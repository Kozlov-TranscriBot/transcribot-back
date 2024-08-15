use std::{env::args, fs::{remove_file, DirBuilder, File}, io::Write};

use axum::{
    extract::{self, DefaultBodyLimit}, 
    http::StatusCode, 
    routing, 
    Router
};
use serde::{Deserialize, Serialize};

use whisper_wrapper::WhisperArgs;

#[derive(Serialize, Deserialize)]
struct RequestQuery {
    id: u32,
    lang: String
}

const TMP_DIR_PATH: &str = "/tmp/transcribot-back";

async fn upload(requset_query: extract::Query<RequestQuery>, mut multipart: extract::Multipart) -> Result<String, (StatusCode, String)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}\n{}", err.body_text(), err.to_string())))?
    {
        if field.name().expect("Field name doesn't found").contains("file") {
            if let Ok(bytes) = field.bytes().await {
                let file_name = format!("{}/{}", TMP_DIR_PATH, requset_query.id);
                let mut audio_file = File::create_new(file_name.clone()).map_err(|_| (
                    StatusCode::TOO_MANY_REQUESTS, String::from("Please wait till previous file will be processed")
                ))?;
                audio_file.write(&bytes).expect("Failed to write data");
                println!("File saved");
                let args: Vec<String> = args().collect();
                let w_args = WhisperArgs::new(
                    args[1].clone(), requset_query.lang.clone(), file_name.clone()
                ).map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
                let res = w_args.run_whisper().map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
                remove_file(file_name).map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
                return Ok(res);
            }
        }
    }

    Err((StatusCode::BAD_REQUEST, String::from("File wasn't recieved")))
}

fn set_router() -> Router {
    Router::new()
        .route("/", routing::post(upload))
        .layer(DefaultBodyLimit::max(21 * 1024 * 1024)) // Max size: 21 MB
}

#[tokio::main]
async fn main() {
    // TODO: Add model size checking

    DirBuilder::new()
        .recursive(true)
        .create(TMP_DIR_PATH).expect("Couldn't create tmp dir");

    let router = set_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
