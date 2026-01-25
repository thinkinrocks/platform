mod endpoints;
mod models;
mod repository;
mod sessions;

use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode, response::Html, routing::get};
use sqlx::sqlite::SqlitePoolOptions;
use tera::{Context, Tera};
use tokio::net::TcpListener;

use crate::{models::Entry, repository::Repository, sessions::Sessions};

async fn index(State(tera): State<Arc<Tera>>) -> Result<Html<String>, StatusCode> {
    let context = Context::new();

    tera.render("index.html", &context)
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn htmx() -> &'static str {
    include_str!("htmx.min.js")
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::query(include_str!("setup.sql"))
        .execute(&pool)
        .await
        .unwrap();

    let repo = Repository::new(pool);
    let sessions = Sessions::new();

    repo.add_entry(Entry {
        id: "USBFLASH-001-100".to_string(),
        name: "USB Flash Memory Stick".to_string(),
        image: None,
        description: None,
        note: None,
        created_at: None,
        stored_in: None,
        responsible_person: None,
    })
    .await
    .unwrap();

    println!("{:#?}", repo.get_entries().await.unwrap());

    let tera = Tera::new("templates/**/*").unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/htmx.min.js", get(htmx))
        .with_state(Arc::new(tera))
        .with_state(repo)
        .with_state(sessions);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
