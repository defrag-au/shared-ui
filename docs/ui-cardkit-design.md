# ui-cardkit Design Document

## Overview

`ui-cardkit` is a Leptos component library for building card game UIs. It provides the visual building blocks for card games without imposing game logic - consumers own their game state and rules.

The primary design target is **Leviathan Hunt** (see `cardkit-sample-game.md`), but the toolkit should support other card game patterns (TCGs, deckbuilders, etc.).

---

## Design Principles

### 1. UI Only, No Game Logic

The toolkit renders cards and zones. It does not:
- Enforce rules (hand limits, valid targets, turn order)
- Manage game state (whose turn, win conditions)
- Handle networking or persistence

Consumers pass state via signals and receive events via callbacks.

### 2. Composition Over Configuration

Small, focused components that compose together rather than monolithic "game board" components with dozens of props.

### 3. Animation-Ready

Card games need fluid animations: draw, play, destroy, shuffle. Components expose hooks for enter/exit animations and position transitions.

### 4. NFT-Aware But Not NFT-Required

Cards can display NFT assets via IIIF integration (leveraging existing `AssetCard`), but also support custom imagery or procedural card faces.

### 5. Theming from the Start

All visual properties exposed via CSS custom properties for easy skinning. Different games can have distinct visual identities.

---

## Visual Architecture: Monster as Gameboard

For Leviathan Hunt, the monster itself **becomes the play surface** rather than sitting on a traditional game board.

### Layout Structure

```
+-----------------------------------------------------+
|                                                     |
|              +---------------------+                |
|              |                     |                |
|              |     LEVIATHAN       |                |
|         [E]  |   (fills play area) |  [E]           |
|              |                     |                |
|              |  [E]          [E]   |                |
|              |         [E]         |                |
|              +---------------------+                |
|                                                     |
|   +---------------------------------------------+   |
|   |  DEPLOYED SUMMARY: Ours (5) | Theirs (12)   |   |
|   +---------------------------------------------+   |
|                                                     |
|   +---+ +---+ +---+ +---+ +---+                     |
|   |   | |   | |   | |   | |   |   YOUR HAND         |
|   +---+ +---+ +---+ +---+ +---+                     |
+-----------------------------------------------------+

[E] = Deployed engine (compact square format)
```

### Key Visual Elements

#### The Monster Surface (`MonsterStage`)
- High-quality monster artwork fills the main play area
- Could have defined zones/attachment points (head, flanks, tail)
- Visual state updates to reflect damage/hunt progress
- Different monsters = different "boards" with unique layouts

#### Deployed Engines (`DeploymentZone` + `CompactCard`)
- Shown as **compact square cards** (similar to AssetCard style)
- Positioned on/around the monster
- Visual distinction between yours and others' deployments
- **Tap/click any engine** → opens full card detail modal

#### Deployment Summary
- Shows cumulative contribution: "Your Engines (5) | Fleet Engines (12)"
- Players care about total force vs the monster, not individual player breakdowns

#### Your Hand (`CardHand`)
- Traditional card hand at bottom of screen
- Full card format for selection
- Tap to deploy → engine moves to monster surface in compact form

---

## Component Inventory

### Core Card Components

#### `GameCard`

The base card display component for full-sized cards (hand, detail view).

```rust
#[component]
pub fn GameCard(
    /// Card dimensions
    #[prop(optional, default = CardSize::Md)]
    size: CardSize,
    /// Visual states
    #[prop(into, optional)]
    highlighted: Option<Signal<bool>>,
    #[prop(into, optional)]
    disabled: Option<Signal<bool>>,
    /// Interaction
    #[prop(into, optional)]
    on_click: Option<Callback<()>>,
    /// Content slots
    children: Children,
) -> impl IntoView
```

#### `CompactCard`

Square format card for deployed engines - shows essential info at a glance.

```rust
#[component]
pub fn CompactCard(
    /// Asset ID for IIIF image
    #[prop(into, optional)]
    asset_id: Option<Signal<String>>,
    /// Direct image URL fallback
    #[prop(into, optional)]
    image_url: Option<Signal<String>>,
    /// Card size (square)
    #[prop(optional, default = CompactSize::Md)]
    size: CompactSize,
    /// Owner indicator (you vs others)
    #[prop(into, optional)]
    owner: Option<Signal<Owner>>,
    /// Click to show detail modal
    #[prop(into, optional)]
    on_click: Option<Callback<()>>,
    /// Overlay stats (power, damage, etc.)
    #[prop(optional)]
    stats: Option<ChildrenFn>,
) -> impl IntoView

pub enum Owner {
    You,
    Other,
}

pub enum CompactSize {
    Sm,  // 48px
    Md,  // 64px
    Lg,  // 80px
}
```

#### `CardDetailModal`

Full card detail view shown when clicking a deployed engine.

```rust
#[component]
pub fn CardDetailModal(
    /// Modal open state
    #[prop(into)]
    open: Signal<bool>,
    /// Close callback
    #[prop(into)]
    on_close: Callback<()>,
    /// Card content
    children: Children,
) -> impl IntoView
```

---

### Stage Components

#### `MonsterStage`

The monster-as-gameboard container. Renders the target monster as the play surface.

```rust
#[component]
pub fn MonsterStage(
    /// Monster image URL
    #[prop(into)]
    monster_image: Signal<String>,
    /// Monster name (for accessibility)
    #[prop(into)]
    monster_name: Signal<String>,
    /// Health/damage state (0.0 - 1.0)
    #[prop(into, optional)]
    health_percent: Option<Signal<f32>>,
    /// Deployed engines overlay
    #[prop(optional)]
    deployments: Option<ChildrenFn>,
    /// Optional status effects/overlays on monster
    #[prop(optional)]
    status_effects: Option<ChildrenFn>,
) -> impl IntoView
```

#### `DeploymentZone`

Container for deployed engines, positioned over the monster.

```rust
#[component]
pub fn DeploymentZone<C, K>(
    /// Deployed cards
    #[prop(into)]
    cards: Signal<Vec<C>>,
    /// Key function
    key_fn: impl Fn(&C) -> K + Clone + 'static,
    /// Render each card
    render_card: impl Fn(C) -> impl IntoView + Clone + 'static,
    /// Layout style
    #[prop(optional, default = DeploymentLayout::Scatter)]
    layout: DeploymentLayout,
) -> impl IntoView

pub enum DeploymentLayout {
    Scatter,      // Random-ish positions around monster
    Grid,         // Organized grid
    Slots(usize), // Fixed slot positions
}
```

#### `DeploymentSummary`

Shows aggregated deployment counts.

```rust
#[component]
pub fn DeploymentSummary(
    /// Your deployed count
    #[prop(into)]
    your_count: Signal<usize>,
    /// Others' deployed count
    #[prop(into)]
    others_count: Signal<usize>,
    /// Optional total power display
    #[prop(into, optional)]
    your_power: Option<Signal<u32>>,
    #[prop(into, optional)]
    others_power: Option<Signal<u32>>,
) -> impl IntoView
```

---

### Hand Components

#### `CardHand`

Displays the player's hand.

```rust
#[component]
pub fn CardHand<C, K>(
    /// Cards in hand
    #[prop(into)]
    cards: Signal<Vec<C>>,
    /// Key function
    key_fn: impl Fn(&C) -> K + Clone + 'static,
    /// Render each card
    render_card: impl Fn(C, HandCardState) -> impl IntoView + Clone + 'static,
    /// Card selected/played
    #[prop(into, optional)]
    on_select: Option<Callback<K>>,
    /// Hand limit for overflow warning
    #[prop(into, optional)]
    max_size: Option<usize>,
) -> impl IntoView

pub struct HandCardState {
    pub index: usize,
    pub is_selected: Signal<bool>,
}
```

**Behaviours:**
- Horizontal layout with optional overlap for many cards
- Selected card lifts/highlights
- Click/tap to select (no drag)
- Overflow state shows visual warning when over limit

---

### Status Components

#### `HealthBar`

For monster health display.

```rust
#[component]
pub fn HealthBar(
    #[prop(into)]
    current: Signal<u32>,
    #[prop(into)]
    max: Signal<u32>,
    #[prop(optional, default = true)]
    show_value: bool,
    #[prop(optional)]
    variant: HealthBarVariant,
) -> impl IntoView
```

#### `StatBadge`

Compact stat display for card overlays.

```rust
#[component]
pub fn StatBadge(
    #[prop(into)]
    value: Signal<String>,
    #[prop(into, optional)]
    icon: Option<String>,
    #[prop(optional)]
    variant: StatVariant,
) -> impl IntoView
```

---

## State Ownership

The toolkit does NOT manage game state. Consumers provide:

```rust
// Consumer's game state (example)
struct LeviathanGameState {
    hand: RwSignal<Vec<EngineCard>>,
    your_deployed: RwSignal<Vec<DeployedEngine>>,
    others_deployed: RwSignal<Vec<DeployedEngine>>,
    monster_health: RwSignal<u32>,
    monster_max_health: RwSignal<u32>,
}

// Consumer wires state to components
view! {
    <MonsterStage
        monster_image=monster_image
        monster_name="Leviathan"
        health_percent=move || {
            let h = game.monster_health.get() as f32;
            let max = game.monster_max_health.get() as f32;
            h / max
        }
        deployments=view! {
            <DeploymentZone
                cards=game.your_deployed
                key_fn=|e| e.id.clone()
                render_card=|e| view! {
                    <CompactCard
                        asset_id=e.asset_id
                        owner=Owner::You
                        on_click=move |_| show_detail(e.id)
                    />
                }
            />
            <DeploymentZone
                cards=game.others_deployed
                key_fn=|e| e.id.clone()
                render_card=|e| view! {
                    <CompactCard
                        asset_id=e.asset_id
                        owner=Owner::Other
                    />
                }
            />
        }
    />

    <DeploymentSummary
        your_count=move || game.your_deployed.get().len()
        others_count=move || game.others_deployed.get().len()
    />

    <CardHand
        cards=game.hand
        key_fn=|c| c.id.clone()
        render_card=|c, state| view! {
            <GameCard highlighted=state.is_selected>
                // card content
            </GameCard>
        }
        on_select=move |card_id| game.deploy(card_id)
    />
}
```

---

## Styling Architecture

### CSS Custom Properties (Theming)

All visual properties exposed for skinning:

```scss
:root {
  // Card dimensions
  --cardkit-card-width: 120px;
  --cardkit-card-ratio: 1.4;  // height = width * ratio
  --cardkit-card-radius: 8px;
  
  // Compact card (deployed)
  --cardkit-compact-size-sm: 48px;
  --cardkit-compact-size-md: 64px;
  --cardkit-compact-size-lg: 80px;
  
  // Colors
  --cardkit-bg: #1a1a2e;
  --cardkit-border: #2a2a4e;
  --cardkit-highlight: rgba(255, 215, 0, 0.6);
  --cardkit-owner-you: rgba(76, 175, 80, 0.3);
  --cardkit-owner-other: rgba(156, 39, 176, 0.3);
  
  // Health bar
  --cardkit-health-bg: #333;
  --cardkit-health-fill: #4caf50;
  --cardkit-health-low: #f44336;
  
  // Animation timing
  --cardkit-transition-fast: 150ms;
  --cardkit-transition-normal: 300ms;
}

// Game-specific theme override
.theme-leviathan {
  --cardkit-highlight: rgba(0, 191, 255, 0.6);
  --cardkit-health-fill: #00bfff;
}
```

### SCSS Structure

```
ui-cardkit/src/styles/
├── mod.scss           # Entry point
├── _variables.scss    # CSS custom properties definitions
├── game_card.scss
├── compact_card.scss
├── monster_stage.scss
├── deployment_zone.scss
├── card_hand.scss
├── health_bar.scss
└── animations.scss
```

---

## Animation System

### CSS-Based Animations

Keep animations in CSS for performance:

```scss
// Card enter animation
.cardkit-card-enter {
  animation: cardkit-card-enter var(--cardkit-transition-normal) ease-out;
}

@keyframes cardkit-card-enter {
  from {
    opacity: 0;
    transform: scale(0.8) translateY(20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

// Deploy animation (hand to stage)
.cardkit-deploy {
  animation: cardkit-deploy var(--cardkit-transition-normal) ease-out;
}

@keyframes cardkit-deploy {
  from {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
  to {
    transform: scale(0.5);  // shrink to compact size
  }
}
```

### Animation Hooks

Components expose lifecycle for custom animations:

```rust
#[component]
pub fn GameCard(
    /// Called when card enters
    #[prop(into, optional)]
    on_enter: Option<Callback<web_sys::HtmlElement>>,
    /// Called before card exits
    #[prop(into, optional)]
    on_exit: Option<Callback<web_sys::HtmlElement>>,
    // ...
) -> impl IntoView
```

---

## Implementation Phases

### Phase 1: Foundation (Current)
- [x] Design document with monster-as-gameboard architecture
- [ ] `ui-cardkit` crate structure
- [ ] `GameCard` - base card component
- [ ] `CompactCard` - deployed engine view
- [ ] Basic SCSS with theming variables

### Phase 2: Stage & Zones
- [ ] `MonsterStage` - monster as gameboard
- [ ] `DeploymentZone` - deployed cards container
- [ ] `DeploymentSummary` - aggregated counts
- [ ] `CardHand` - player's hand

### Phase 3: Interactions & Details
- [ ] `CardDetailModal` - full card on click
- [ ] Selection/highlight states
- [ ] `HealthBar` and `StatBadge`

### Phase 4: Polish
- [ ] Enter/exit animations
- [ ] Deploy animation (hand → stage)
- [ ] Storybook stories
- [ ] Documentation

---

## Storybook Stories

`_storybook` should include:

1. **Card Gallery** - GameCard and CompactCard in all sizes/states
2. **Monster Stage Demo** - Static monster with deployed engines
3. **Hand Interaction** - Select cards from hand
4. **Full Game Layout** - Complete Leviathan Hunt prototype
5. **Theming** - Multiple theme examples

---

## Open Questions (Resolved)

1. ~~Card content model~~ → Slot-driven initially, free placement later
2. ~~Drag-and-drop~~ → Click/tap only, no drag mechanics
3. ~~Multi-player view~~ → Cumulative deployed view (yours vs theirs)
4. ~~Animation system~~ → CSS-based with lifecycle hooks
5. ~~Theming~~ → CSS custom properties from the start
