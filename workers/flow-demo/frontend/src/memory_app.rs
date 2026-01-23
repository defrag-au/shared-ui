//! Memory Game Application
//!
//! The main entry point for the Black Flag memory game frontend.

use crate::components::{
    AdminPanel, CardView, GameBoard, GameMode, GameResults, Lobby, PlayerInfo, PlayerList,
};
use crate::{get_or_create_user_id, ConnectionStatus};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ui_flow_protocol::{ClientMessage, OpId, PresenceInfo, ServerMessage};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, MessageEvent, WebSocket};

// =============================================================================
// Types mirroring the server types
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerGameMode {
    #[default]
    TurnTaking,
    Race,
}

impl From<ServerGameMode> for GameMode {
    fn from(m: ServerGameMode) -> Self {
        match m {
            ServerGameMode::TurnTaking => GameMode::TurnTaking,
            ServerGameMode::Race => GameMode::Race,
        }
    }
}

impl From<GameMode> for ServerGameMode {
    fn from(m: GameMode) -> Self {
        match m {
            GameMode::TurnTaking => ServerGameMode::TurnTaking,
            GameMode::Race => ServerGameMode::Race,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameConfig {
    pub grid_size: (u8, u8),
    pub mode: ServerGameMode,
    pub policy_id: String,
    pub flip_delay_ms: u64,
    pub shuffle_seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum GamePhase {
    Lobby {
        min_players: u8,
        max_players: u8,
    },
    Starting {
        countdown: u8,
    },
    Playing,
    Finished {
        winner: Option<String>,
        rankings: Vec<(String, String, u32)>,
    },
}

impl Default for GamePhase {
    fn default() -> Self {
        Self::Lobby {
            min_players: 1,
            max_players: 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub asset_id: String,
    pub name: String,
    pub image_url: String,
    pub matched: bool,
    pub matched_by: Option<String>,
    pub pair_id: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub user_id: String,
    pub user_name: String,
    pub score: u32,
    pub flipped: Vec<usize>,
    pub spectating: bool,
    pub joined_at: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryGameState {
    pub config: GameConfig,
    pub cards: Vec<Card>,
    pub phase: GamePhase,
    pub players: HashMap<String, PlayerState>,
    pub turn_order: Vec<String>,
    pub current_turn: usize,
    pub shared_flipped: Vec<usize>,
    pub flip_time: Option<u64>,
    pub host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardFace {
    pub asset_id: String,
    pub name: String,
}

/// Hidden card data - just the structure, no revealed info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenCard {
    pub matched: bool,
    pub matched_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemoryDelta {
    PlayerJoined {
        user_id: String,
        user_name: String,
        spectating: bool,
    },
    PlayerLeft {
        user_id: String,
    },
    ConfigChanged {
        config: GameConfig,
    },
    HostChanged {
        user_id: String,
    },
    CountdownTick {
        seconds: u8,
    },
    GameStarted {
        turn_order: Vec<String>,
    },
    CardsDealt {
        cards: Vec<HiddenCard>,
    },
    CardFlipped {
        index: usize,
        by: String,
        face: CardFace,
    },
    TurnChanged {
        user_id: String,
    },
    OwnCardFlipped {
        index: usize,
        face: CardFace,
    },
    PairMatched {
        indices: [usize; 2],
        by: String,
        by_name: String,
        new_score: u32,
        face: CardFace,
    },
    CardsReset {
        indices: [usize; 2],
        for_player: Option<String>,
    },
    ScoreChanged {
        user_id: String,
        score: u32,
    },
    GameEnded {
        winner: Option<String>,
        rankings: Vec<(String, String, u32)>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemoryEvent {
    StartingIn {
        seconds: u8,
    },
    MatchFound {
        player_name: String,
        pair_name: String,
    },
    Streak {
        player_name: String,
        count: u8,
    },
    NearVictory {
        player_name: String,
        pairs_remaining: u8,
    },
    InvalidAction {
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemoryAction {
    JoinGame {
        user_name: String,
    },
    LeaveGame,
    SetConfig {
        mode: Option<ServerGameMode>,
        grid_size: Option<(u8, u8)>,
    },
    StartGame,
    FlipCard {
        index: usize,
    },
    RequestRematch,
    ResetGame,
}

type ServerMsg = ServerMessage<MemoryGameState, MemoryDelta, MemoryEvent>;
type ClientMsg = ClientMessage<MemoryAction>;

// =============================================================================
// Memory Game App Component
// =============================================================================

/// Memory Game Application component
#[component]
pub fn MemoryApp() -> impl IntoView {
    let user_id = get_or_create_user_id();
    let user_id_for_ws = user_id.clone();

    let (room_id, _set_room_id) = create_signal("default".to_string());
    let (status, set_status) = create_signal(ConnectionStatus::Disconnected);
    let (game_state, set_game_state) = create_signal(MemoryGameState::default());
    let (presence, set_presence) = create_signal(Vec::<PresenceInfo>::new());
    let (current_user_id, _) = create_signal(user_id);

    // Local UI state for flipped cards (before server confirms)
    let (local_flipped, set_local_flipped) = create_signal(Vec::<usize>::new());
    // Revealed card faces (from server)
    let (revealed_faces, set_revealed_faces) = create_signal(HashMap::<usize, CardFace>::new());

    let ws: Rc<RefCell<Option<WebSocket>>> = Rc::new(RefCell::new(None));

    // Disconnect helper
    let ws_disconnect = ws.clone();
    let disconnect = Rc::new(move || {
        if let Some(socket) = ws_disconnect.borrow_mut().take() {
            let _ = socket.close();
        }
        set_status.set(ConnectionStatus::Disconnected);
    });

    // Connect to WebSocket
    let ws_connect = ws.clone();
    let user_id_ws = user_id_for_ws.clone();
    let connect = Rc::new(move || {
        let room = room_id.get();
        let ws_url = get_memory_ws_url(&room, &user_id_ws);

        set_status.set(ConnectionStatus::Connecting);

        match WebSocket::new(&ws_url) {
            Ok(socket) => {
                socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

                let set_status_clone = set_status;
                let on_open = Closure::wrap(Box::new(move |_: JsValue| {
                    set_status_clone.set(ConnectionStatus::Connected);
                    tracing::info!("Memory game WebSocket connected");
                }) as Box<dyn FnMut(JsValue)>);
                socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
                on_open.forget();

                let set_status_clone = set_status;
                let on_close = Closure::wrap(Box::new(move |_: JsValue| {
                    set_status_clone.set(ConnectionStatus::Disconnected);
                    tracing::info!("Memory game WebSocket disconnected");
                }) as Box<dyn FnMut(JsValue)>);
                socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));
                on_close.forget();

                let on_error = Closure::wrap(Box::new(move |e: JsValue| {
                    tracing::error!("Memory game WebSocket error: {:?}", e);
                }) as Box<dyn FnMut(JsValue)>);
                socket.set_onerror(Some(on_error.as_ref().unchecked_ref()));
                on_error.forget();

                let set_game_state_clone = set_game_state;
                let set_presence_clone = set_presence;
                let set_revealed_clone = set_revealed_faces;
                let set_local_flipped_clone = set_local_flipped;
                let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(buffer) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        let array = js_sys::Uint8Array::new(&buffer);
                        let bytes = array.to_vec();

                        match ui_flow_protocol::decode::<ServerMsg>(&bytes) {
                            Ok(msg) => {
                                handle_server_message(
                                    msg,
                                    set_game_state_clone,
                                    set_presence_clone,
                                    set_revealed_clone,
                                    set_local_flipped_clone,
                                );
                            }
                            Err(e) => {
                                tracing::error!("Failed to decode message: {}", e);
                            }
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
                socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
                on_message.forget();

                *ws_connect.borrow_mut() = Some(socket);
            }
            Err(e) => {
                tracing::error!("Failed to create WebSocket: {:?}", e);
                set_status.set(ConnectionStatus::Disconnected);
            }
        }
    });

    // Send action helper
    let ws_send = ws.clone();
    let send_action = Rc::new(move |action: MemoryAction| {
        if let Some(socket) = ws_send.borrow().as_ref() {
            if socket.ready_state() == WebSocket::OPEN {
                let msg: ClientMsg = ClientMessage::action(OpId::new(), action);
                if let Ok(bytes) = ui_flow_protocol::encode(&msg) {
                    let _ = socket.send_with_u8_array(&bytes);
                }
            }
        }
    });
    let send_action_for_admin = send_action.clone();

    // Auto-connect on mount
    let connect_effect = connect.clone();
    create_effect(move |_| {
        connect_effect();
    });

    // Auto-join game when connected
    let send_join = send_action.clone();
    let user_id_join = current_user_id.get();
    create_effect(move |prev_status: Option<ConnectionStatus>| {
        let current = status.get();
        if prev_status != Some(ConnectionStatus::Connected)
            && current == ConnectionStatus::Connected
        {
            // Send join action
            let display_name = format!("User {}", &user_id_join[..8.min(user_id_join.len())]);
            send_join(MemoryAction::JoinGame {
                user_name: display_name,
            });
        }
        current
    });

    // Derived signals
    let game_mode = Signal::derive(move || game_state.get().config.mode.into());
    let grid_size = Signal::derive(move || game_state.get().config.grid_size);

    let players_for_lobby = Signal::derive(move || {
        let state = game_state.get();
        state
            .players
            .values()
            .map(|p| (p.user_id.clone(), p.user_name.clone()))
            .collect::<Vec<_>>()
    });

    let players_for_list = Signal::derive(move || {
        let state = game_state.get();
        state
            .players
            .values()
            .map(|p| PlayerInfo {
                user_id: p.user_id.clone(),
                user_name: p.user_name.clone(),
                score: p.score,
                spectating: p.spectating,
            })
            .collect::<Vec<_>>()
    });

    let current_turn_user = Signal::derive(move || {
        let state = game_state.get();
        state.turn_order.get(state.current_turn).cloned()
    });

    let is_my_turn = {
        let current_user_id = current_user_id;
        Signal::derive(move || {
            let state = game_state.get();
            let my_id = current_user_id.get();

            // In race mode, it's always "your turn" if you're playing
            if state.config.mode == ServerGameMode::Race {
                return state
                    .players
                    .get(&my_id)
                    .map(|p| !p.spectating)
                    .unwrap_or(false);
            }

            // In turn-taking mode, check if it's actually your turn
            state.turn_order.get(state.current_turn) == Some(&my_id)
        })
    };

    let cards_view = {
        let _current_user_id = current_user_id;
        Signal::derive(move || {
            let state = game_state.get();
            let revealed = revealed_faces.get();
            let _my_id = _current_user_id.get();
            let local = local_flipped.get();

            state
                .cards
                .iter()
                .enumerate()
                .map(|(idx, card)| {
                    // Card is visible if:
                    // 1. It's matched
                    // 2. It's in revealed_faces (server confirmed flip)
                    // 3. In turn-taking mode and it's in shared_flipped
                    let is_matched = card.matched;
                    let is_revealed = revealed.contains_key(&idx);
                    let is_shared_flipped = state.config.mode == ServerGameMode::TurnTaking
                        && state.shared_flipped.contains(&idx);
                    let is_local_flipped = local.contains(&idx);

                    let visible =
                        is_matched || is_revealed || is_shared_flipped || is_local_flipped;

                    // Get face data if visible - use asset_id for IIIF URL generation
                    let (asset_id, name) = if visible {
                        if let Some(face) = revealed.get(&idx) {
                            (Some(face.asset_id.clone()), Some(face.name.clone()))
                        } else if is_matched {
                            (Some(card.asset_id.clone()), Some(card.name.clone()))
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    };

                    CardView {
                        index: idx,
                        visible,
                        asset_id,
                        name,
                        matched: is_matched,
                        matched_by: card.matched_by.clone(),
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    let flipped_indices = Signal::derive(move || {
        let state = game_state.get();
        let local = local_flipped.get();
        let mut indices = state.shared_flipped.clone();
        for idx in local {
            if !indices.contains(&idx) {
                indices.push(idx);
            }
        }
        indices
    });

    // Clone for view handlers
    let connect_reconnect = connect.clone();

    let status_str = move || match status.get() {
        ConnectionStatus::Connected => "connected",
        ConnectionStatus::Connecting => "connecting",
        ConnectionStatus::Disconnected => "disconnected",
    };

    view! {
        <div class="memory-app">
            <div class="header">
                <h1>"Black Flag Memory"</h1>
                <connection-status
                    attr:status=status_str
                    on:reconnect=move |_: CustomEvent| {
                        connect_reconnect();
                    }
                />
            </div>

            {move || {
                let state = game_state.get();

                match state.phase {
                    GamePhase::Lobby { .. } => {
                        let send = send_action.clone();
                        let send_mode = send_action.clone();
                        let send_grid = send_action.clone();
                        let send_start = send_action.clone();

                        view! {
                            <Lobby
                                players=players_for_lobby
                                current_user_id=current_user_id
                                game_mode=game_mode
                                grid_size=grid_size
                                on_mode_change=move |mode: GameMode| {
                                    send_mode(MemoryAction::SetConfig {
                                        mode: Some(mode.into()),
                                        grid_size: None,
                                    });
                                }
                                on_grid_change=move |size: (u8, u8)| {
                                    send_grid(MemoryAction::SetConfig {
                                        mode: None,
                                        grid_size: Some(size),
                                    });
                                }
                                on_start=move || {
                                    send_start(MemoryAction::StartGame);
                                }
                            />
                        }.into_view()
                    }

                    GamePhase::Starting { countdown } => {
                        view! {
                            <div class="countdown">
                                <h2>"Game starting in..."</h2>
                                <div class="countdown-number">{countdown}</div>
                            </div>
                        }.into_view()
                    }

                    GamePhase::Playing => {
                        let send_flip = send_action.clone();

                        view! {
                            <div class="game-playing">
                                <div class="game-main">
                                    <GameBoard
                                        grid_size=grid_size
                                        cards=cards_view
                                        flipped_indices=flipped_indices
                                        is_my_turn=is_my_turn
                                        on_flip=move |idx: usize| {
                                            // Optimistically add to local flipped
                                            set_local_flipped.update(|v| {
                                                if !v.contains(&idx) && v.len() < 2 {
                                                    v.push(idx);
                                                }
                                            });
                                            send_flip(MemoryAction::FlipCard { index: idx });
                                        }
                                        disabled=Signal::derive(move || status.get() != ConnectionStatus::Connected)
                                    />
                                </div>
                                <div class="game-sidebar">
                                    <PlayerList
                                        players=players_for_list
                                        current_turn=current_turn_user
                                        current_user_id=current_user_id
                                    />
                                </div>
                            </div>
                        }.into_view()
                    }

                    GamePhase::Finished { winner, rankings } => {
                        let send_rematch = send_action.clone();
                        let (winner_sig, _) = create_signal(winner);
                        let (rankings_sig, _) = create_signal(rankings);

                        view! {
                            <GameResults
                                winner=winner_sig
                                rankings=rankings_sig
                                current_user_id=current_user_id
                                on_rematch=move || {
                                    send_rematch(MemoryAction::RequestRematch);
                                }
                            />
                        }.into_view()
                    }
                }
            }}

            // Admin panel
            {
                let send_reset = send_action_for_admin.clone();
                view! {
                    <AdminPanel on_reset=move || {
                        send_reset(MemoryAction::ResetGame);
                    } />
                }
            }
        </div>
    }
}

fn handle_server_message(
    msg: ServerMsg,
    set_game_state: WriteSignal<MemoryGameState>,
    set_presence: WriteSignal<Vec<PresenceInfo>>,
    set_revealed_faces: WriteSignal<HashMap<usize, CardFace>>,
    set_local_flipped: WriteSignal<Vec<usize>>,
) {
    match msg {
        ServerMessage::Connected { .. } => {
            tracing::info!("Received Connected message");
        }

        ServerMessage::Snapshot { state, seq, .. } => {
            tracing::info!("Received Snapshot at seq {}", seq);
            set_game_state.set(state);
            // Clear local state on snapshot
            set_revealed_faces.set(HashMap::new());
            set_local_flipped.set(Vec::new());
        }

        ServerMessage::Delta { delta, seq, .. } => {
            tracing::debug!("Received Delta at seq {}", seq);
            apply_delta(delta, set_game_state, set_revealed_faces, set_local_flipped);
        }

        ServerMessage::Deltas { deltas, seq, .. } => {
            tracing::debug!("Received {} Deltas, final seq {}", deltas.len(), seq);
            for delta in deltas {
                apply_delta(delta, set_game_state, set_revealed_faces, set_local_flipped);
            }
        }

        ServerMessage::Presence { users } => {
            tracing::debug!("Received Presence: {} users", users.len());
            set_presence.set(users);
        }

        ServerMessage::Notify { domain, event, .. } => {
            tracing::debug!("Received Notify on domain: {}", domain);
            match event {
                MemoryEvent::MatchFound {
                    player_name,
                    pair_name,
                } => {
                    tracing::info!("{} found a match: {}", player_name, pair_name);
                }
                MemoryEvent::InvalidAction { reason } => {
                    tracing::warn!("Invalid action: {}", reason);
                }
                _ => {}
            }
        }

        ServerMessage::ActionOk { op_id, .. } => {
            tracing::debug!("Action {} completed successfully", op_id);
        }

        ServerMessage::ActionErr { op_id, message, .. } => {
            tracing::error!("Action {} failed: {}", op_id, message);
            // Clear local flipped on error
            set_local_flipped.set(Vec::new());
        }

        _ => {}
    }
}

fn apply_delta(
    delta: MemoryDelta,
    set_game_state: WriteSignal<MemoryGameState>,
    set_revealed_faces: WriteSignal<HashMap<usize, CardFace>>,
    set_local_flipped: WriteSignal<Vec<usize>>,
) {
    match delta {
        MemoryDelta::PlayerJoined {
            user_id,
            user_name,
            spectating,
        } => {
            set_game_state.update(|s| {
                s.players.insert(
                    user_id.clone(),
                    PlayerState {
                        user_id,
                        user_name,
                        score: 0,
                        flipped: vec![],
                        spectating,
                        joined_at: 0,
                    },
                );
            });
        }

        MemoryDelta::PlayerLeft { user_id } => {
            set_game_state.update(|s| {
                s.players.remove(&user_id);
            });
        }

        MemoryDelta::ConfigChanged { config } => {
            set_game_state.update(|s| {
                s.config = config;
            });
        }

        MemoryDelta::HostChanged { user_id } => {
            set_game_state.update(|s| {
                s.host = Some(user_id);
            });
        }

        MemoryDelta::CountdownTick { seconds } => {
            set_game_state.update(|s| {
                s.phase = GamePhase::Starting { countdown: seconds };
            });
        }

        MemoryDelta::GameStarted { turn_order } => {
            set_game_state.update(|s| {
                s.turn_order = turn_order;
                s.current_turn = 0;
                s.phase = GamePhase::Playing;
            });
            set_revealed_faces.set(HashMap::new());
            set_local_flipped.set(Vec::new());
        }

        MemoryDelta::CardsDealt { cards } => {
            // Convert hidden cards to full Card structs (face data hidden)
            set_game_state.update(|s| {
                s.cards = cards
                    .into_iter()
                    .map(|hidden| Card {
                        asset_id: String::new(), // Hidden until flipped
                        name: String::new(),     // Hidden until flipped
                        image_url: String::new(),
                        matched: hidden.matched,
                        matched_by: hidden.matched_by,
                        pair_id: 0, // Not needed on client
                    })
                    .collect();
            });
        }

        MemoryDelta::CardFlipped { index, by: _, face } => {
            // Add to revealed faces
            set_revealed_faces.update(|m| {
                m.insert(index, face);
            });
            // Update shared_flipped
            set_game_state.update(|s| {
                if !s.shared_flipped.contains(&index) {
                    s.shared_flipped.push(index);
                }
            });
            // Clear from local flipped (server confirmed)
            set_local_flipped.update(|v| {
                v.retain(|&i| i != index);
            });
        }

        MemoryDelta::TurnChanged { user_id } => {
            set_game_state.update(|s| {
                if let Some(idx) = s.turn_order.iter().position(|id| id == &user_id) {
                    s.current_turn = idx;
                }
            });
        }

        MemoryDelta::OwnCardFlipped { index, face } => {
            // Race mode: only we see this
            set_revealed_faces.update(|m| {
                m.insert(index, face);
            });
            set_local_flipped.update(|v| {
                v.retain(|&i| i != index);
            });
        }

        MemoryDelta::PairMatched {
            indices,
            by,
            by_name: _,
            new_score,
            face: _,
        } => {
            set_game_state.update(|s| {
                // Mark cards as matched
                for &idx in &indices {
                    if let Some(card) = s.cards.get_mut(idx) {
                        card.matched = true;
                        card.matched_by = Some(by.clone());
                    }
                }
                // Update score
                if let Some(player) = s.players.get_mut(&by) {
                    player.score = new_score;
                }
                // Clear shared flipped
                s.shared_flipped.clear();
            });
            // Clear revealed faces for these cards (they stay visible as matched)
            set_revealed_faces.update(|m| {
                for &idx in &indices {
                    m.remove(&idx);
                }
            });
            set_local_flipped.set(Vec::new());
        }

        MemoryDelta::CardsReset {
            indices,
            for_player: _,
        } => {
            // Clear revealed faces
            set_revealed_faces.update(|m| {
                for &idx in &indices {
                    m.remove(&idx);
                }
            });
            // Clear shared flipped
            set_game_state.update(|s| {
                s.shared_flipped.clear();
            });
            set_local_flipped.set(Vec::new());
        }

        MemoryDelta::ScoreChanged { user_id, score } => {
            set_game_state.update(|s| {
                if let Some(player) = s.players.get_mut(&user_id) {
                    player.score = score;
                }
            });
        }

        MemoryDelta::GameEnded { winner, rankings } => {
            set_game_state.update(|s| {
                s.phase = GamePhase::Finished { winner, rankings };
            });
        }
    }
}

fn get_memory_ws_url(room_id: &str, user_id: &str) -> String {
    let window = web_sys::window().expect("no window");
    let location = window.location();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".to_string());
    let host = location
        .host()
        .unwrap_or_else(|_| "localhost:8787".to_string());

    let ws_protocol = if protocol == "https:" { "wss:" } else { "ws:" };
    let display_name = &user_id[..8.min(user_id.len())];

    format!(
        "{}//{}/memory/{}?user_id={}&user_name={}",
        ws_protocol, host, room_id, user_id, display_name
    )
}
