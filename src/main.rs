mod cacher;
mod trace_middleware;

use tide::{Request, Response, StatusCode};
use tide_acme::{AcmeConfig, TideRustlsExt};
use trace_middleware::TraceMiddleware;

async fn find_content(key: &str) -> tide::Result {
    // 0. Check if exists in index
    todo!();
    // 1. Check in-memory cache ¿taken as argument?
    todo!();
    // 2. Check on-disk ¿where?
    todo!();
    // 3. Check in block store ¿where?
    todo!();
}

async fn start() {
    let port = std::env::var("PORT").unwrap();

    let mut app = tide::new();

    app.with(TraceMiddleware);
    app.at("/favicon.ico").get(|_| async {
        Ok(Response::builder(StatusCode::Ok)
            .body_bytes(include_bytes!("../static_assets/favicon/favicon.ico"))
            .content_type("image/vnd.microsoft.icon")
            .build())
    });
    app.at("/:file").get(|req: Request<()>| async move {
        let key = req.param("file")?;
        find_content(key).await
    });

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
