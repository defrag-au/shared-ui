//! CompactCard Component
//!
//! Square format card for deployed engines. Shows essential info at a glance
//! with minimal footprint. Used for cards deployed on the monster stage.
//!
//! ## Usage
//!
//! ```ignore
//! <CompactCard
//!     asset_id=engine.asset_id
//!     size=CompactSize::Md
//!     owner=Owner::You
//!     on_click=move |_| show_detail_modal()
//!     stats=view! {
//!         <StatBadge value="350" icon="⚡" />
//!     }
//! />
//! ```

use leptos::children::ChildrenFn;
use leptos::prelude::*;
use ui_components::{generate_iiif_url, IiifSize, ImageCard};

/// Size variants for compact (deployed) cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompactSize {
    /// 48px - dense grid
    Sm,
    /// 64px - default deployed size
    #[default]
    Md,
    /// 80px - larger deployed view
    Lg,
}

impl CompactSize {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            CompactSize::Sm => "sm",
            CompactSize::Md => "md",
            CompactSize::Lg => "lg",
        }
    }
}

/// Owner indicator for deployed cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Owner {
    /// Your deployed card
    #[default]
    You,
    /// Another player's deployed card
    Other,
}

impl Owner {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Owner::You => "you",
            Owner::Other => "other",
        }
    }
}

/// Compact square card for deployed engines
#[component]
pub fn CompactCard(
    /// Cardano asset ID for IIIF image
    #[prop(into, optional)]
    asset_id: Option<Signal<String>>,
    /// Direct image URL (fallback)
    #[prop(into, optional)]
    image_url: Option<Signal<String>>,
    /// Card size
    #[prop(optional, default = CompactSize::Md)]
    size: CompactSize,
    /// Owner indicator
    #[prop(into, optional)]
    owner: Option<Signal<Owner>>,
    /// Click callback (typically opens detail modal)
    #[prop(into, optional)]
    on_click: Option<Callback<()>>,
    /// Overlay stats (power, damage, etc.)
    #[prop(optional)]
    stats: Option<ChildrenFn>,
) -> impl IntoView {
    let size_class = format!("cardkit-compact-card--{}", size.class_suffix());
    let is_clickable = on_click.is_some();

    // Resolve image URL - prefer direct URL, fall back to IIIF
    let resolved_url: Memo<String> = Memo::new(move |_| {
        // Direct image URL takes precedence
        if let Some(ref url_signal) = image_url {
            let url = url_signal.get();
            if !url.is_empty() {
                return url;
            }
        }

        // Fall back to IIIF URL generation
        if let Some(ref id_signal) = asset_id {
            let id = id_signal.get();
            if !id.is_empty() {
                if let Some(url) = generate_iiif_url(&id, IiifSize::Thumb) {
                    return url;
                }
            }
        }

        String::new()
    });

    let class_string = move || {
        let mut classes = vec!["cardkit-compact-card", &size_class];

        if let Some(o) = owner {
            match o.get() {
                Owner::You => classes.push("cardkit-compact-card--owner-you"),
                Owner::Other => classes.push("cardkit-compact-card--owner-other"),
            }
        }

        if is_clickable {
            classes.push("cardkit-compact-card--clickable");
        }

        classes.join(" ")
    };

    let handle_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };

    // Eagerly render stats
    let stats_content = stats.map(|s| s());

    view! {
        <div
            class=class_string
            on:click=handle_click
        >
            <div class="cardkit-compact-card__image">
                <ImageCard
                    image_url=Signal::derive(move || resolved_url.get())
                    is_static=true
                    show_name=false
                    show_skeleton=true
                />
            </div>

            {stats_content.map(|content| view! {
                <div class="cardkit-compact-card__stats">
                    {content}
                </div>
            })}
        </div>
    }
}
