//! # flow-demo
//!
//! Demo worker showcasing the unified realtime protocol with a Leptos frontend.
//!
//! This worker demonstrates:
//! - WebSocket connections via Durable Objects
//! - MessagePack binary protocol
//! - Snapshot + delta state synchronization
//! - Presence tracking
//! - Optimistic UI with action feedback

mod session;
mod types;

pub use session::FlowDemoSessionDO;

use worker::*;

/// Initialize tracing and panic hook
#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
}

/// Main fetch handler
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get("/health", |_, _| Response::ok("OK"))
        .get_async("/ws/:room_id", handle_websocket)
        .run(req, env)
        .await
}

/// Handle WebSocket upgrade request by routing to the appropriate Durable Object
async fn handle_websocket(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let room_id = ctx
        .param("room_id")
        .map(|s| s.as_str())
        .unwrap_or("default");

    // Get the Durable Object namespace
    let namespace = ctx.env.durable_object("FLOW_SESSIONS")?;

    // Get or create the DO instance for this room
    let stub = namespace.id_from_name(room_id)?.get_stub()?;

    // Forward the WebSocket upgrade request to the DO
    stub.fetch_with_request(req).await
}
