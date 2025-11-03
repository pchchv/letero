use serde::Serialize;
use std::{fmt::Display, ops::Deref, sync::Arc};
use axum::{extract::Request, middleware::Next, response::Response};

#[derive(Serialize, Clone, Debug)]
pub struct TraceId(Arc<String>);

impl TraceId {
    pub fn new() -> TraceId {
        TraceId(Arc::new(small_uid::SmallUid::new().to_string()))
    }
}

impl Display for TraceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for TraceId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn trace(mut req: Request, next: Next) -> Response {
    let trace_id = TraceId::new();
    let span = tracing::info_span!(
        "request",
        %trace_id,
        method = %req.method(),
        path = %req.uri().path(),
    );
    let _enter = span.enter();

    tracing::trace!("request started");

    req.extensions_mut().insert(trace_id);

    tracing::trace!("trace id added to request extensions");

    let response = next.run(req).await;

    tracing::trace!("request completed");

    response
}