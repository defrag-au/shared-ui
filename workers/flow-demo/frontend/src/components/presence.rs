//! Presence component

use leptos::*;
use ui_flow_protocol::PresenceInfo;

/// Online users list
#[component]
pub fn Presence(
    /// List of online users
    users: ReadSignal<Vec<PresenceInfo>>,
) -> impl IntoView {
    view! {
        <div class="card presence">
            <h2>"Online Users (" {move || users.get().len()} ")"</h2>
            <Show
                when=move || !users.get().is_empty()
                fallback=|| view! { <p class="empty">"No users online"</p> }
            >
                <ul class="user-list">
                    <For
                        each=move || users.get()
                        key=|user| user.user_id.clone()
                        children=move |user| {
                            let name = user.name.clone().unwrap_or_else(|| user.user_id.clone());
                            view! {
                                <li>
                                    <span class="status-dot"></span>
                                    <span class="name">{name}</span>
                                </li>
                            }
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}
