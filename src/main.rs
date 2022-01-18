mod trace_middleware;

use tide_acme::{AcmeConfig, TideRustlsExt};
use trace_middleware::TraceMiddleware;

async fn start() {
    let port = std::env::var("PORT").unwrap();

    let mut app = tide::new();

    app.with(TraceMiddleware);
    app.at("/").get(|_| async { Ok("Hello world!") });

    app.listen(
        tide_rustls::TlsListener::build()
            .addrs(format!("0.0.0.0:{port}"))
            .acme(
                AcmeConfig::new()
                    .domains(vec![std::env::var("DOMAIN_NAME").unwrap()])
                    .contact_email("funder@fastmail.com")
                    .cache_dir("/certs")
                    .production(),
            ),
    )
    .await
    .unwrap();
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    async_std::task::block_on(async { start().await })
}
