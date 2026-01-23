//! UI Flow stories - overview, FlowState trait, operations

use super::helpers::{
    render_config_option_card, render_flow_concept_card, render_loader_step_card,
};
use primitives::{create_element, AppendChild};
use web_sys::Element;

// ============================================================================
// Flow Overview Story
// ============================================================================

pub fn render_flow_overview_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("UI Flow Overview"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Real-time state synchronization using the snapshot + delta pattern over WebSockets.",
    ));
    header.append(&desc);
    container.append(&header);

    // Core concepts section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Core Concepts"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    let c1 = render_flow_concept_card(
        "Snapshot",
        "Full state sent on initial connection. Client replaces local state entirely.",
    );
    grid.append(&c1);

    let c2 = render_flow_concept_card(
        "Delta",
        "Incremental updates sent after snapshot. Client applies changes to existing state.",
    );
    grid.append(&c2);

    let c3 = render_flow_concept_card(
        "FlowState",
        "Trait for state types that can receive deltas via apply_delta() method.",
    );
    grid.append(&c3);

    let c4 = render_flow_concept_card(
        "OpId",
        "Operation ID for tracking client actions through their lifecycle (optimistic UI).",
    );
    grid.append(&c4);

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Protocol flow section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Protocol Flow"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid2 = create_element("div", &["story-grid"]);

    let s1 = render_loader_step_card(
        "1",
        "Connect",
        "Client opens WebSocket connection to server",
    );
    grid2.append(&s1);

    let s2 = render_loader_step_card(
        "2",
        "Snapshot",
        "Server sends full state snapshot with sequence number",
    );
    grid2.append(&s2);

    let s3 = render_loader_step_card(
        "3",
        "Deltas",
        "Server sends incremental updates as state changes",
    );
    grid2.append(&s3);

    let s4 = render_loader_step_card(
        "4",
        "Actions",
        "Client sends actions with OpId for optimistic UI tracking",
    );
    grid2.append(&s4);

    canvas2.append(&grid2);
    section2.append(&canvas2);
    container.append(&section2);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Quick Start"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use ui_flow::{FlowConnection, FlowState, OpId};

// Connect with callbacks
let conn = FlowConnection::<MyState, MyAction>::builder()
    .url("wss://example.com/realtime")
    .on_snapshot(|state, seq| {
        // Replace local state
    })
    .on_delta(|delta, seq| {
        // Apply incremental update
    })
    .on_status(|status| {
        // Update connection indicator
    })
    .connect()?;

// Send action with tracking
let op_id = OpId::new();
conn.send_action(op_id, MyAction::DoSomething)?;"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Flow State Story
// ============================================================================

pub fn render_flow_state_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("FlowState Trait"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Define how your state type receives incremental updates from the server.",
    ));
    header.append(&desc);
    container.append(&header);

    // Trait definition
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Trait Definition"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"pub trait FlowState: Clone + Default {
    /// The delta type for incremental updates
    type Delta: DeserializeOwned;

    /// Apply a delta to update the state
    fn apply_delta(&mut self, delta: Self::Delta);
}"#,
    ));
    canvas.append(&code);
    section.append(&canvas);
    container.append(&section);

    // Implementation example
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Implementation Example"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let code2 = create_element("pre", &["code-block"]);
    code2.set_text_content(Some(
        r#"use ui_flow::FlowState;
use std::collections::HashMap;

#[derive(Clone, Default, Deserialize)]
struct GameState {
    entities: HashMap<String, Entity>,
    tick: u64,
}

#[derive(Deserialize)]
enum GameDelta {
    EntityUpdated { id: String, entity: Entity },
    EntityRemoved { id: String },
    TickAdvanced { tick: u64 },
}

impl FlowState for GameState {
    type Delta = GameDelta;

    fn apply_delta(&mut self, delta: Self::Delta) {
        match delta {
            GameDelta::EntityUpdated { id, entity } => {
                self.entities.insert(id, entity);
            }
            GameDelta::EntityRemoved { id } => {
                self.entities.remove(&id);
            }
            GameDelta::TickAdvanced { tick } => {
                self.tick = tick;
            }
        }
    }
}"#,
    ));
    canvas2.append(&code2);
    section2.append(&canvas2);
    container.append(&section2);

    // Benefits section
    let section3 = create_element("div", &["story-section"]);
    let h3_3 = create_element("h3", &[]);
    h3_3.set_text_content(Some("Benefits"));
    section3.append(&h3_3);

    let canvas3 = create_element("div", &["story-canvas"]);
    let grid3 = create_element("div", &["story-grid"]);

    let b1 = render_flow_concept_card(
        "Bandwidth Efficient",
        "Only changed data is sent after initial snapshot, reducing network traffic.",
    );
    grid3.append(&b1);

    let b2 = render_flow_concept_card(
        "Type Safe",
        "Compile-time guarantees that deltas match state structure.",
    );
    grid3.append(&b2);

    let b3 = render_flow_concept_card(
        "Framework Agnostic",
        "Works with any UI framework - Seed, Leptos, Yew, or vanilla.",
    );
    grid3.append(&b3);

    canvas3.append(&grid3);
    section3.append(&canvas3);
    container.append(&section3);

    container
}

// ============================================================================
// Flow Operations Story
// ============================================================================

pub fn render_flow_operations_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Operation Tracking"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Track client actions through their lifecycle for optimistic UI updates.",
    ));
    header.append(&desc);
    container.append(&header);

    // OpId section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("OpId"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    let card1 = render_config_option_card(
        "OpId::new()",
        "OpId",
        "-",
        "Generate a unique operation ID for tracking an action",
    );
    grid.append(&card1);

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Operation lifecycle
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Operation Lifecycle"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid2 = create_element("div", &["story-grid"]);

    let s1 = render_loader_step_card("1", "Start", "Client sends action with OpId");
    grid2.append(&s1);

    let s2 = render_loader_step_card("2", "Progress", "Server sends ActionProgress updates");
    grid2.append(&s2);

    let s3 = render_loader_step_card(
        "3",
        "Complete",
        "Server sends ActionComplete or ActionError",
    );
    grid2.append(&s3);

    canvas2.append(&grid2);
    section2.append(&canvas2);
    container.append(&section2);

    // OperationTracker section
    let section3 = create_element("div", &["story-section"]);
    let h3_3 = create_element("h3", &[]);
    h3_3.set_text_content(Some("OperationTracker"));
    section3.append(&h3_3);

    let canvas3 = create_element("div", &["story-canvas"]);
    let code3 = create_element("pre", &["code-block"]);
    code3.set_text_content(Some(
        r#"use ui_flow::{OperationTracker, OpId};

let mut tracker = OperationTracker::new();

// Start tracking when sending an action
let op_id = tracker.start(MyActionData { item_id: 42 });
connection.send_action(op_id, Action::BuyItem { id: 42 })?;

// Show optimistic UI immediately
show_pending_purchase(42);

// On progress update
connection.on_progress(|progress| {
    tracker.update_progress(progress);
    update_progress_bar(progress.percent);
});

// On completion
connection.on_action_complete(|op_id| {
    if let Some(data) = tracker.complete(op_id) {
        confirm_purchase(data.item_id);
    }
});

// On error - rollback optimistic update
connection.on_action_error(|error| {
    if let Some(data) = tracker.fail(&error) {
        rollback_purchase(data.item_id);
        show_error(error.message);
    }
});"#,
    ));
    canvas3.append(&code3);
    section3.append(&canvas3);
    container.append(&section3);

    // Types section
    let section4 = create_element("div", &["story-section"]);
    let h3_4 = create_element("h3", &[]);
    h3_4.set_text_content(Some("Related Types"));
    section4.append(&h3_4);

    let canvas4 = create_element("div", &["story-canvas"]);
    let grid4 = create_element("div", &["story-grid"]);

    let t1 = render_config_option_card(
        "ActionProgress",
        "struct",
        "-",
        "Contains op_id, optional percent (0-100), and optional message",
    );
    grid4.append(&t1);

    let t2 = render_config_option_card(
        "ActionError",
        "struct",
        "-",
        "Contains op_id, optional error code, and error message",
    );
    grid4.append(&t2);

    let t3 = render_config_option_card(
        "PendingOperation<T>",
        "struct",
        "-",
        "Wraps your action data with op_id, progress, and timestamp",
    );
    grid4.append(&t3);

    canvas4.append(&grid4);
    section4.append(&canvas4);
    container.append(&section4);

    container
}
