use crate::{
    AppState,
    services::auth::Auth,
    models::events::SseEvent,
};
use axum::{
    Extension,
    extract::State,
    response::{Sse, sse::Event},
};
use std::sync::Arc;
use tokio::sync::broadcast;
use futures_util::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

/// Get chat events
#[utoipa::path(
    get,
    path = "/events",
    tag = "events",
    responses(
        (status = 200, description = "OK", content_type = "text/event-stream"),
    ),
    security(("auth" = []))
)]
pub async fn events(
    Extension(auth): Extension<Arc<Auth>>,
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let rx = state
        .events
        .entry(auth.user.id)
        .or_insert_with(|| broadcast::channel(16).0)
        .value()
        .subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(SseEvent { event_type, data }) => {
                tracing::trace!("SSE event {} emitted", event_type.as_ref());
                Some(Ok(Event::default().event(event_type.as_ref()).data(data)))
            }
            
            Err(err) => {
                tracing::error!("SSE error: {err}");
                None
            }
        }
    });

    Sse::new(stream)
}
