use std::{collections::HashMap, env::args, fs::File, io::Read};

use axum::{extract, http::response, response::Html, routing, Router};
use tower_http::services::ServeDir;
use serde::Deserialize;

#[derive(Deserialize)]
struct UserContent {
    id: u64,
    content: String,
}

async fn handle_audio(
    extract::Json(data): extract::Json<UserContent>
) -> Html<String> {
    Html(String::from("OK"))
}

fn set_router() -> Router {
    Router::new()
        .route("/", routing::post(handle_audio))
}

#[tokio::main]
async fn main() {
    let router = set_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
