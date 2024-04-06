use std::collections::HashMap;

use askama::Template;
//use axum::routing::post;
use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use generation::BlogPostConst;
//use tower_http::services::{ServeDir, ServeFile};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod html_template;
mod posts;
use crate::html_template::HtmlTemplate;
use crate::posts::POSTS;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // populate hashmap for blogposts
    let mut post_mapper: HashMap<String, BlogPostConst> = HashMap::new();
    for p in POSTS {
        post_mapper.insert(p.metadata.url.to_string(), p.clone());
    }

    // set up tracing for logging with defaults
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jackbranchdev=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    println!("Hello, world!");
    let app = Router::new()
        .route("/", get(index))
        .route("/blog/:file_name", get(blog));

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

async fn blog(Path(file_name): Path<String>) -> impl IntoResponse {
    // TODO: include blogpost mapper in AppState and access it here
    HtmlTemplate(BlogTemplate { title: file_name })
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String, //post: BlogPost,
}
