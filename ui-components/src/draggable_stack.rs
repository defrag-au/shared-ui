//! Draggable Stack Component
//!
//! A container for drag-and-drop reorderable items with visual drop indicators.
//!
//! ## Features
//!
//! - Horizontal or vertical layout
//! - Drop position indicators (lines between items)
//! - Gap placeholder where dragged item originated
//! - Automatic drag event wiring
//! - Customizable rendering via render function
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
//!             <div
//!                 class="my-item"
//!                 class:dragging=drag_state.is_source
//!             >
//!                 {item}
//!             </div>
//!         }
//!     />
//! }
//! ```

use crate::{use_draggable, Reorder};
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

    fn indicator_style(&self) -> &'static str {
        match self {
            StackDirection::Horizontal => "width: 3px; height: 100%; left: -2px; top: 0;",
            StackDirection::Vertical => "height: 3px; width: 100%; top: -2px; left: 0;",
        }
    }

    fn end_indicator_style(&self) -> &'static str {
        match self {
            StackDirection::Horizontal => "width: 3px; height: 100%; right: -2px; top: 0;",
            StackDirection::Vertical => "height: 3px; width: 100%; bottom: -2px; left: 0;",
        }
    }
}

/// Drag state passed to render function
#[derive(Clone, Copy)]
pub struct ItemDragState {
    /// True if this item is currently being dragged
    pub is_source: bool,
    /// True if drag is active (any item being dragged)
    pub drag_active: bool,
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
    /// Receives (item, index, drag_state)
    render_item: F,
    /// Additional class for the container
    #[prop(into, optional)]
    class: String,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(T, usize, ItemDragState) -> V + Clone + Send + Sync + 'static,
    V: IntoView + 'static,
{
    // Use the draggable hook
    let draggable = use_draggable(move |reorder| {
        on_reorder.run(reorder);
    });

    // Track which position the drop indicator should show at
    // None = no indicator, Some(idx) = show before item at idx
    // Special case: idx == items.len() means show at end
    let (drop_target_position, set_drop_target_position) = signal(None::<usize>);

    let container_class = if class.is_empty() {
        "ui-draggable-stack".to_string()
    } else {
        format!("ui-draggable-stack {class}")
    };

    let container_style = format!(
        "display: flex; flex-direction: {}; gap: {gap};",
        direction.flex_direction()
    );

    view! {
        <div class=container_class style=container_style>
            {
                let draggable = draggable.clone();
                let render_item = render_item.clone();

                view! {
                    <For
                        each={move || items.get().into_iter().enumerate().collect::<Vec<_>>()}
                        key=|(idx, _)| *idx
                        children=move |(idx, item)| {
                            let draggable = draggable.clone();
                            let render_item = render_item.clone();
                            let item_for_render = item.clone();

                            // Get drag state for this item
                            let drag_state = {
                                let draggable = draggable.clone();
                                move || ItemDragState {
                                    is_source: draggable.is_dragging(idx),
                                    drag_active: draggable.is_active(),
                                }
                            };

                            // Get the drag attrs
                            let attrs = draggable.attrs(idx);
                            let draggable_for_events = draggable.clone();

                            // Calculate drop position based on mouse position within item
                            let on_drag_over_with_position = {
                                move |ev: web_sys::DragEvent| {
                                    ev.prevent_default();

                                    // Get the drag state
                                    let state = draggable_for_events.state().get_untracked();
                                    if state.source_index.is_none() {
                                        return;
                                    }
                                    let source_idx = state.source_index.unwrap();

                                    // Determine if we're in the first or second half of the element
                                    if let Some(target) = ev.current_target() {
                                        if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                                            let rect = element.get_bounding_client_rect();
                                            let is_before = match direction {
                                                StackDirection::Horizontal => {
                                                    let mid = rect.left() + rect.width() / 2.0;
                                                    ev.client_x() as f64 <= mid
                                                }
                                                StackDirection::Vertical => {
                                                    let mid = rect.top() + rect.height() / 2.0;
                                                    ev.client_y() as f64 <= mid
                                                }
                                            };

                                            // Calculate drop position
                                            let drop_pos = if is_before { idx } else { idx + 1 };

                                            // Don't show indicator at source position or adjacent
                                            // (dropping there would be a no-op)
                                            if drop_pos == source_idx || drop_pos == source_idx + 1 {
                                                set_drop_target_position.set(None);
                                            } else {
                                                set_drop_target_position.set(Some(drop_pos));
                                            }
                                        }
                                    }
                                }
                            };

                            let on_drag_leave = move |_: web_sys::DragEvent| {
                                // Clear indicator when leaving (will be set again if entering another)
                                set_drop_target_position.set(None);
                            };

                            let attrs_for_drop = attrs.clone();
                            let on_drop_handler = move |ev: web_sys::DragEvent| {
                                set_drop_target_position.set(None);
                                attrs_for_drop.on_drop(ev);
                            };

                            let attrs_for_end = attrs.clone();
                            let on_drag_end_handler = move |ev: web_sys::DragEvent| {
                                set_drop_target_position.set(None);
                                attrs_for_end.on_drag_end(ev);
                            };

                            let attrs_for_start = attrs;

                            // Show drop indicator before this item?
                            let show_indicator_before = move || {
                                drop_target_position.get() == Some(idx)
                            };

                            // For the last item, also check if indicator should show after
                            let items_len = items.get().len();
                            let is_last = idx == items_len.saturating_sub(1);
                            let show_indicator_after = move || {
                                is_last && drop_target_position.get() == Some(idx + 1)
                            };

                            let indicator_style = direction.indicator_style();
                            let end_indicator_style = direction.end_indicator_style();

                            // Is this item being dragged?
                            let draggable_for_source = draggable.clone();
                            let draggable_for_opacity = draggable.clone();

                            view! {
                                <div
                                    class="ui-draggable-stack__item-wrapper"
                                    class:ui-draggable-stack__item-wrapper--dragging=move || draggable_for_source.is_dragging(idx)
                                    style="position: relative;"
                                    draggable="true"
                                    on:dragstart=move |ev| attrs_for_start.on_drag_start(ev)
                                    on:dragend=on_drag_end_handler
                                    on:dragover=on_drag_over_with_position
                                    on:dragleave=on_drag_leave
                                    on:drop=on_drop_handler
                                >
                                    // Drop indicator BEFORE this item
                                    <div
                                        class="ui-draggable-stack__indicator"
                                        class:ui-draggable-stack__indicator--visible=show_indicator_before
                                        style=format!("position: absolute; {indicator_style} background: var(--ui-draggable-indicator-color, #3b82f6); border-radius: 2px; pointer-events: none; opacity: 0; transition: opacity 0.15s;")
                                    />

                                    // The actual item content
                                    <div
                                        class="ui-draggable-stack__item"
                                        style=move || if draggable_for_opacity.is_dragging(idx) { "opacity: 0.4;" } else { "" }
                                    >
                                        {render_item(item_for_render.clone(), idx, drag_state())}
                                    </div>

                                    // Drop indicator AFTER this item (only for last item)
                                    {is_last.then(|| view! {
                                        <div
                                            class="ui-draggable-stack__indicator"
                                            class:ui-draggable-stack__indicator--visible=show_indicator_after
                                            style=format!("position: absolute; {end_indicator_style} background: var(--ui-draggable-indicator-color, #3b82f6); border-radius: 2px; pointer-events: none; opacity: 0; transition: opacity 0.15s;")
                                        />
                                    })}
                                </div>
                            }
                        }
                    />
                }
            }
        </div>
    }
}
