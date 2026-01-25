//! Draggable Stack Component
//!
//! A container for drag-and-drop reorderable items using mouse events.
//!
//! ## Features
//!
//! - Horizontal or vertical layout
//! - Smooth drag with actual element movement
//! - Items shift to show drop position
//! - Container-based mouse capture for reliable tracking
//!
//! ## Usage
//!
//! ```ignore
//! use ui_components::{DraggableStack, StackDirection};
//!
//! let (items, set_items) = signal(vec!["A", "B", "C", "D"]);
//!
//! view! {
//!     <DraggableStack
//!         items=items
//!         on_reorder=move |reorder| set_items.update(|i| reorder.apply(i))
//!         direction=StackDirection::Horizontal
//!         gap="0.5rem"
//!         render_item=move |item, _idx, drag_state| view! {
//!             <div class="my-item" class:dragging=drag_state.is_source>
//!                 {item}
//!             </div>
//!         }
//!     />
//! }
//! ```

use crate::Reorder;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Stack layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StackDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl StackDirection {
    fn flex_direction(&self) -> &'static str {
        match self {
            StackDirection::Horizontal => "row",
            StackDirection::Vertical => "column",
        }
    }
}

/// Drag state passed to render function
#[derive(Clone, Copy, Default)]
pub struct ItemDragState {
    /// True if this item is currently being dragged
    pub is_source: bool,
    /// True if drag is active (any item being dragged)
    pub drag_active: bool,
}

/// Internal drag state
#[derive(Clone, Default)]
struct DragState {
    /// Index of item being dragged
    source_index: Option<usize>,
    /// Current target position (where item would be inserted)
    target_position: Option<usize>,
    /// Current mouse offset from drag start
    offset_x: f64,
    offset_y: f64,
    /// Starting mouse position
    start_x: f64,
    start_y: f64,
}

/// Draggable stack component for reorderable lists
#[component]
pub fn DraggableStack<T, F, V>(
    /// Items to render
    #[prop(into)]
    items: Signal<Vec<T>>,
    /// Callback when items are reordered
    #[prop(into)]
    on_reorder: Callback<Reorder>,
    /// Stack direction
    #[prop(optional, default = StackDirection::Horizontal)]
    direction: StackDirection,
    /// Gap between items (CSS value)
    #[prop(into, optional, default = "0.5rem".into())]
    gap: String,
    /// Render function for each item
    render_item: F,
    /// Additional class for the container
    #[prop(into, optional)]
    class: String,
) -> impl IntoView
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
    F: Fn(T, usize, ItemDragState) -> V + Clone + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let (drag_state, set_drag_state) = signal(DragState::default());
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let container_class = if class.is_empty() {
        "ui-draggable-stack".to_string()
    } else {
        format!("ui-draggable-stack {class}")
    };

    let container_style = format!(
        "display: flex; flex-direction: {}; gap: {gap}; position: relative; user-select: none;",
        direction.flex_direction()
    );

    // Mouse down on an item - start drag
    let on_item_mousedown = move |idx: usize, ev: web_sys::MouseEvent| {
        ev.prevent_default();
        set_drag_state.set(DragState {
            source_index: Some(idx),
            target_position: Some(idx),
            offset_x: 0.0,
            offset_y: 0.0,
            start_x: ev.client_x() as f64,
            start_y: ev.client_y() as f64,
        });
    };

    // Mouse move on container - update drag position
    let on_container_mousemove = move |ev: web_sys::MouseEvent| {
        let state = drag_state.get();
        if state.source_index.is_none() {
            return;
        }

        let source_idx = state.source_index.unwrap();
        let offset_x = ev.client_x() as f64 - state.start_x;
        let offset_y = ev.client_y() as f64 - state.start_y;

        // Calculate target position based on mouse position
        let target_pos = if let Some(container) = container_ref.get() {
            let container_el: &web_sys::HtmlElement = &container;
            let children = container_el.children();
            let mouse_pos = match direction {
                StackDirection::Horizontal => ev.client_x() as f64,
                StackDirection::Vertical => ev.client_y() as f64,
            };

            let mut new_pos = source_idx;
            let item_count = items.get_untracked().len();

            for i in 0..children.length() {
                if let Some(child) = children.item(i) {
                    if let Ok(el) = child.dyn_into::<web_sys::HtmlElement>() {
                        let rect = el.get_bounding_client_rect();
                        let mid = match direction {
                            StackDirection::Horizontal => rect.left() + rect.width() / 2.0,
                            StackDirection::Vertical => rect.top() + rect.height() / 2.0,
                        };

                        if mouse_pos < mid {
                            new_pos = i as usize;
                            break;
                        } else {
                            new_pos = (i as usize + 1).min(item_count);
                        }
                    }
                }
            }
            Some(new_pos)
        } else {
            state.target_position
        };

        set_drag_state.set(DragState {
            source_index: Some(source_idx),
            target_position: target_pos,
            offset_x,
            offset_y,
            start_x: state.start_x,
            start_y: state.start_y,
        });
    };

    // Mouse up - complete drag
    let on_container_mouseup = move |_ev: web_sys::MouseEvent| {
        let state = drag_state.get();
        if let (Some(source), Some(target)) = (state.source_index, state.target_position) {
            if source != target && source + 1 != target {
                on_reorder.run(Reorder::new(source, target));
            }
        }
        set_drag_state.set(DragState::default());
    };

    // Mouse leave container - cancel drag
    let on_container_mouseleave = move |_ev: web_sys::MouseEvent| {
        set_drag_state.set(DragState::default());
    };

    view! {
        <div
            class=container_class
            style=container_style
            node_ref=container_ref
            on:mousemove=on_container_mousemove
            on:mouseup=on_container_mouseup
            on:mouseleave=on_container_mouseleave
        >
            <For
                each={move || items.get().into_iter().enumerate().collect::<Vec<_>>()}
                key=|(_, item)| item.clone()
                children=move |(idx, item)| {
                    let render_item = render_item.clone();
                    let item_for_render = item.clone();

                    // Calculate transform and styles for this item
                    let item_style = move || {
                        let state = drag_state.get();
                        let source = state.source_index;
                        let target = state.target_position;

                        if source == Some(idx) {
                            // This is the dragged item - apply offset transform
                            let (tx, ty) = match direction {
                                StackDirection::Horizontal => (state.offset_x, 0.0),
                                StackDirection::Vertical => (0.0, state.offset_y),
                            };
                            format!(
                                "transform: translate({tx}px, {ty}px); z-index: 100; position: relative; \
                                 transition: none; cursor: grabbing;"
                            )
                        } else if let (Some(src), Some(tgt)) = (source, target) {
                            // Other items may need to shift
                            let shift = calculate_shift(idx, src, tgt, direction);
                            if shift != 0.0 {
                                let (tx, ty) = match direction {
                                    StackDirection::Horizontal => (shift, 0.0),
                                    StackDirection::Vertical => (0.0, shift),
                                };
                                format!(
                                    "transform: translate({tx}px, {ty}px); \
                                     transition: transform 0.15s ease; cursor: grab;"
                                )
                            } else {
                                "transition: transform 0.15s ease; cursor: grab;".to_string()
                            }
                        } else {
                            "cursor: grab;".to_string()
                        }
                    };

                    let drag_state_for_render = move || {
                        let state = drag_state.get();
                        ItemDragState {
                            is_source: state.source_index == Some(idx),
                            drag_active: state.source_index.is_some(),
                        }
                    };

                    view! {
                        <div
                            class="ui-draggable-stack__item-wrapper"
                            style=item_style
                            on:mousedown=move |ev| on_item_mousedown(idx, ev)
                        >
                            {render_item(item_for_render.clone(), idx, drag_state_for_render())}
                        </div>
                    }
                }
            />
        </div>
    }
}

/// Calculate the shift amount for an item based on drag state
fn calculate_shift(idx: usize, source: usize, target: usize, _direction: StackDirection) -> f64 {
    // Estimate item size + gap (will be refined by actual measurements)
    let item_size = 70.0; // Approximate - ideally we'd measure

    if source < target {
        // Dragging forward: items between source+1 and target-1 shift backward
        if idx > source && idx < target {
            return -item_size;
        }
    } else if source > target {
        // Dragging backward: items between target and source-1 shift forward
        if idx >= target && idx < source {
            return item_size;
        }
    }
    0.0
}
