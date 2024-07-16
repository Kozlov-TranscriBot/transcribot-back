use axum::{extract, response::Html, routing, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserContent {
    id: u32,
    content: String,
}

async fn handle_audio(
    extract::Json(data): extract::Json<UserContent>
) -> Html<String> {
    Html(format!("Content {} from user {}", data.content, data.id))
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
