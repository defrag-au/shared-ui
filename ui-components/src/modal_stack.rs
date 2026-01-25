//! Modal Stack Component
//!
//! A modal with view swapping support for nested interactions.
//! Views slide in/out horizontally with breadcrumb navigation.
//!
//! ## Design
//!
//! Instead of stacking multiple modals, this component maintains a stack of views
//! within a single modal container. Child views can push new views onto the stack,
//! and the user can navigate back via breadcrumbs or a back button.
//!
//! ## Nested Modals
//!
//! When a `<Modal>` is rendered inside a ModalStack, it automatically detects
//! the `ModalNavigation` context and coordinates with the stack. The Modal renders
//! its content inline but the stack manages breadcrumbs and navigation.
//!
//! ## Usage
//!
//! ```ignore
//! #[derive(Clone, PartialEq)]
//! enum MyView {
//!     Main,
//!     AddItem,
//!     SelectAsset { filter: String },
//! }
//!
//! <ModalStack
//!     open=show_modal
//!     initial_view=MyView::Main
//!     on_close=Callback::new(move |()| set_show_modal.set(false))
//!     view_title=|view: &MyView| match view {
//!         MyView::Main => "My Modal".into(),
//!         MyView::AddItem => "Add Item".into(),
//!         MyView::SelectAsset { .. } => "Select Asset".into(),
//!     }
//!     view_content=|view: MyView, ctx: ModalStackContext<MyView>| match view {
//!         MyView::Main => view! {
//!             <button on:click={
//!                 let ctx = ctx.clone();
//!                 move |_| ctx.push(MyView::AddItem)
//!             }>"Add"</button>
//!         }.into_any(),
//!         // ... etc
//!     }
//! />
//! ```

use crate::modal_context::{ModalNavigation, ModalViewId, MountedView};
use leptos::prelude::*;
use std::sync::Arc;

/// Context provided to view content for navigation
#[derive(Clone)]
pub struct ModalStackContext<V: Clone + 'static> {
    push_fn: Arc<dyn Fn(V) + Send + Sync>,
    pop_fn: Arc<dyn Fn() + Send + Sync>,
    close_fn: Arc<dyn Fn() + Send + Sync>,
}

impl<V: Clone + 'static> ModalStackContext<V> {
    /// Push a new view onto the stack
    pub fn push(&self, view: V) {
        (self.push_fn)(view);
    }

    /// Pop the current view and go back
    pub fn pop(&self) {
        (self.pop_fn)();
    }

    /// Close the entire modal
    pub fn close(&self) {
        (self.close_fn)();
    }
}

/// Direction of the current slide animation
#[derive(Clone, Copy, PartialEq, Default)]
enum SlideDirection {
    #[default]
    None,
    Forward,
    Back,
}

/// Internal view representation that can be either typed or mounted
#[derive(Clone)]
enum StackEntry<V: Clone> {
    /// A typed view from the view_content function
    Typed(V),
    /// A mounted view from a nested Modal (content renders inline in the Modal)
    Mounted(MountedView),
}

impl<V: Clone> StackEntry<V> {
    fn title<F: Fn(&V) -> String>(&self, typed_title_fn: &F) -> String {
        match self {
            StackEntry::Typed(v) => typed_title_fn(v),
            StackEntry::Mounted(m) => m.title.clone(),
        }
    }

    fn mounted_id(&self) -> Option<ModalViewId> {
        match self {
            StackEntry::Mounted(m) => Some(m.id),
            _ => None,
        }
    }
}

/// Modal with view stack support
///
/// Generic over `V` - the view enum type that defines possible views.
#[component]
pub fn ModalStack<V, TitleFn, ContentFn>(
    /// Signal controlling modal visibility
    #[prop(into)]
    open: Signal<bool>,
    /// Initial view to display
    initial_view: V,
    /// Callback when modal should close
    #[prop(into, optional)]
    on_close: Option<Callback<()>>,
    /// Function to get title for a view
    view_title: TitleFn,
    /// Remove body padding (for full-bleed content)
    #[prop(optional)]
    flush: bool,
    /// Render function for view content - receives current view and navigation context
    view_content: ContentFn,
) -> impl IntoView
where
    V: Clone + PartialEq + Send + Sync + 'static,
    TitleFn: Fn(&V) -> String + Send + Sync + Clone + 'static,
    ContentFn: Fn(V, ModalStackContext<V>) -> AnyView + Send + Sync + Clone + 'static,
{
    // View stack - stores all views (typed and mounted) for preservation
    let (view_stack, set_view_stack) = signal(vec![StackEntry::Typed(initial_view.clone())]);

    // Animation state
    let (slide_direction, set_slide_direction) = signal(SlideDirection::None);
    let (is_animating, set_is_animating) = signal(false);

    // Reset stack when modal closes and reopens
    let initial_for_effect = initial_view.clone();
    Effect::new(move |prev_open: Option<bool>| {
        let currently_open = open.get();
        if currently_open && prev_open == Some(false) {
            // Modal just opened - reset to initial view
            set_view_stack.set(vec![StackEntry::Typed(initial_for_effect.clone())]);
            set_slide_direction.set(SlideDirection::None);
        }
        currently_open
    });

    // Create the context for child views (typed navigation)
    let push_typed_fn = Arc::new(move |view: V| {
        if is_animating.get() {
            return;
        }
        set_slide_direction.set(SlideDirection::Forward);
        set_is_animating.set(true);
        set_view_stack.update(|stack| stack.push(StackEntry::Typed(view)));
    });

    let pop_fn = Arc::new(move || {
        if is_animating.get() {
            return;
        }
        let stack = view_stack.get();
        if stack.len() > 1 {
            set_slide_direction.set(SlideDirection::Back);
            set_is_animating.set(true);
            set_view_stack.update(|stack| {
                stack.pop();
            });
        }
    });

    let close_fn = Arc::new(move || {
        if let Some(cb) = on_close {
            cb.run(());
        }
    });

    let ctx = ModalStackContext {
        push_fn: push_typed_fn,
        pop_fn: pop_fn.clone(),
        close_fn: close_fn.clone(),
    };

    // Create ModalNavigation context for nested Modals
    let mount_fn = Arc::new(move |title: String| {
        let id = ModalViewId::new();
        let mounted = MountedView { id, title };

        if !is_animating.get() {
            set_slide_direction.set(SlideDirection::Forward);
            set_is_animating.set(true);
        }
        set_view_stack.update(|stack| stack.push(StackEntry::Mounted(mounted)));

        id
    });

    let unmount_fn = {
        let pop_fn = pop_fn.clone();
        Arc::new(move |id: ModalViewId| {
            // Check if this ID is on the top of the stack
            let stack = view_stack.get();
            if let Some(entry) = stack.last() {
                if entry.mounted_id() == Some(id) {
                    // It's on top, just pop
                    pop_fn();
                    return;
                }
            }
            // Not on top - remove from stack directly
            if !is_animating.get() {
                set_slide_direction.set(SlideDirection::Back);
                set_is_animating.set(true);
            }
            set_view_stack.update(|stack| {
                stack.retain(|entry| entry.mounted_id() != Some(id));
            });
        })
    };

    let modal_nav =
        ModalNavigation::new(move |title| (mount_fn)(title), move |id| (unmount_fn)(id));

    // Provide the context immediately
    provide_context(modal_nav);

    // Clone for use in view
    let view_title_for_breadcrumbs = view_title.clone();
    let content_fn = view_content.clone();
    let ctx_for_content = ctx.clone();

    // Handle animation end
    let on_animation_end = move |_| {
        set_is_animating.set(false);
        set_slide_direction.set(SlideDirection::None);
    };

    // Close handlers
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Escape" {
            if let Some(cb) = on_close {
                cb.run(());
            }
        }
    };

    let handle_backdrop_click = move |_| {
        if let Some(cb) = on_close {
            cb.run(());
        }
    };

    let stop_propagation = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    let backdrop_style = move || {
        if open.get() {
            "display: flex;"
        } else {
            "display: none;"
        }
    };

    // Animation class
    let animation_class = move || match slide_direction.get() {
        SlideDirection::None => "",
        SlideDirection::Forward => "ui-modal-stack__content--slide-in",
        SlideDirection::Back => "ui-modal-stack__content--slide-out",
    };

    view! {
        <div
            class="ui-modal-backdrop"
            style=backdrop_style
            on:click=handle_backdrop_click
            on:keydown=handle_keydown
            tabindex="-1"
        >
            <div
                class="ui-modal ui-modal-stack"
                on:click=stop_propagation
                role="dialog"
                aria-modal="true"
            >
                // Header with breadcrumbs
                <div class="ui-modal__header ui-modal-stack__header">
                    // Back button (only show when depth > 1)
                    {move || {
                        let stack = view_stack.get();
                        let ctx = ctx.clone();
                        (stack.len() > 1).then(|| view! {
                            <button
                                class="ui-modal-stack__back"
                                on:click=move |_| ctx.pop()
                                aria-label="Go back"
                            >
                                "\u{2190}"
                            </button>
                        })
                    }}

                    // Breadcrumbs
                    <nav class="ui-modal-stack__breadcrumbs">
                        {move || {
                            let stack = view_stack.get();
                            let view_title = view_title_for_breadcrumbs.clone();
                            stack.iter().enumerate().map(|(i, entry)| {
                                let title = entry.title(&view_title);
                                let is_last = i == stack.len() - 1;
                                let target_depth = i + 1;
                                view! {
                                    <span class="ui-modal-stack__breadcrumb">
                                        {if is_last {
                                            view! {
                                                <span class="ui-modal-stack__breadcrumb-current">{title}</span>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <button
                                                    class="ui-modal-stack__breadcrumb-link"
                                                    on:click={
                                                        let set_view_stack = set_view_stack.clone();
                                                        move |_| {
                                                            set_slide_direction.set(SlideDirection::Back);
                                                            set_is_animating.set(true);
                                                            set_view_stack.update(|stack| {
                                                                stack.truncate(target_depth);
                                                            });
                                                        }
                                                    }
                                                >
                                                    {title}
                                                </button>
                                                <span class="ui-modal-stack__breadcrumb-sep">" / "</span>
                                            }.into_any()
                                        }}
                                    </span>
                                }
                            }).collect_view()
                        }}
                    </nav>

                    // Close button
                    {on_close.map(|cb| view! {
                        <button
                            class="ui-modal__close"
                            on:click=move |_| cb.run(())
                            aria-label="Close"
                        >
                            "\u{00D7}"
                        </button>
                    })}
                </div>

                // Content area with slide animation
                <div
                    class="ui-modal__body ui-modal-stack__body"
                    class:flush=flush
                >
                    <div
                        class=move || format!("ui-modal-stack__content {}", animation_class())
                        on:animationend=on_animation_end
                    >
                        {move || {
                            let stack = view_stack.get();
                            let content_fn = content_fn.clone();
                            let ctx = ctx_for_content.clone();

                            // Render all views but only show the current one
                            // This preserves state for previous views
                            stack.iter().enumerate().map(|(i, entry)| {
                                let is_current = i == stack.len() - 1;
                                let content = match entry {
                                    StackEntry::Typed(v) => content_fn(v.clone(), ctx.clone()),
                                    StackEntry::Mounted(_) => {
                                        // Mounted content renders inline via the nested Modal
                                        // We just provide a container that CSS will show/hide
                                        view! {
                                            <div class="ui-modal-stack__mounted-view"></div>
                                        }.into_any()
                                    }
                                };
                                view! {
                                    <div
                                        class="ui-modal-stack__view"
                                        class:ui-modal-stack__view--current=is_current
                                        class:ui-modal-stack__view--hidden=!is_current
                                    >
                                        {content}
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
