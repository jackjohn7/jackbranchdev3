use askama::Template;
//use axum::routing::post;
use axum::{response::IntoResponse, routing::get, Router};
//use tower_http::services::{ServeDir, ServeFile};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod html_template;
use crate::html_template::HtmlTemplate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set up tracing for logging with defaults
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "seek_ql=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    println!("Hello, world!");
    let app = Router::new().route("/", get(index));

    let port = 5173_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("Router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to listen");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve router");

    Ok(())
}

async fn index() -> impl IntoResponse {
    debug!("hit index!");
    HtmlTemplate(IndexTemplate {})
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
