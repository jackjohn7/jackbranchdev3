use std::collections::HashMap;
use std::sync::Arc;

use askama::Template;
use axum::extract::State;
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

struct AppState {
    posts: HashMap<String, BlogPostConst>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // populate hashmap for blogposts
    let mut post_mapper: HashMap<String, BlogPostConst> = HashMap::new();
    for p in POSTS {
        post_mapper.insert(p.metadata.url.to_string(), p.clone());
    }

    let state = AppState { posts: post_mapper };

    // set up tracing for logging with defaults
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jackbranchdev=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/blog/:file_name", get(blog))
        .with_state(Arc::new(state));

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
    debug!("Serving root route");
    HtmlTemplate(IndexTemplate {})
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn blog(
    State(state): State<Arc<AppState>>,
    Path(file_name): Path<String>,
) -> axum::http::Response<axum::body::Body> {
    debug!("Serving blog route");
    match state.posts.get(&file_name) {
        Some(post) => HtmlTemplate(BlogTemplate {
            post: post.clone(),
            time_to_read: ((post.html.split(" ").count() as f32 / 238.0).round()) as i16,
        })
        .into_response(),
        None => HtmlTemplate(IndexTemplate {}).into_response(), // 404 later
    }
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    post: BlogPostConst,
    time_to_read: i16,
}
