use std::{future::Future, pin::Pin, time::Instant};

use tracing::{info, info_span, Instrument};

use tide::{Middleware, Next, Request};

pub struct TraceMiddleware;

impl TraceMiddleware {
    async fn log<'a, State: Clone + Send + Sync + 'static>(
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> tide::Result {
        let start_time = Instant::now();

        let request_id = uuid::Uuid::from_bytes(rand::random());
        let ip = ctx.peer_addr().unwrap_or("");
        let method = ctx.method();
        let route = ctx.url().as_str();
        let span = info_span!(
            "request",
            %request_id,
            http.method = ?method,
            http.url = route,
            net.peer.ip = ip
        );

        let response = next.run(ctx).instrument(span.clone()).await;

        let _span_guard = span.enter();
        let duration = start_time.elapsed();
        let status = response.status() as u16;
        info!(http.duration = ?duration, http.status = status);

        Ok(response)
    }
}

impl<State: Clone + Send + Sync + 'static> Middleware<State> for TraceMiddleware {
    fn handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        request: tide::Request<State>,
        next: tide::Next<'life1, State>,
    ) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(Self::log(request, next))
    }
}
