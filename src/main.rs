mod templates;

use askama::Template;
use axum::{Router, response::Html, routing::get};
use tracing::{info, warn};

use crate::templates::IndexHTML;

async fn htmx() -> &'static str {
    include_str!("htmx.js")
}

async fn index() -> Html<String> {
    Html(IndexHTML.render().unwrap())
}

#[tokio::main]
async fn main() {
    if let Err(err) = dotenvy::dotenv() {
        warn!("Failed to load the environment file is everything configured correctly? Error: {:?}", err);
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();


    let app = Router::new()
        .route("/", get(index))
        .route("/htmx.js", get(htmx));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Serving at http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
