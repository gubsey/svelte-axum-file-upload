use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use base64::Engine;
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(file_handler))
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any));

    axum::Server::bind(&"0.0.0.0:1971".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct Model {
    file_data: String,
}

async fn file_handler(
    Json(Model { file_data }): Json<Model>,
) -> Result<String, (StatusCode, String)> {
    let raw_base64 = file_data.split(',').last().ok_or((
        StatusCode::BAD_REQUEST,
        "Could not parse data url".to_string(),
    ))?;
    let engine = base64::engine::general_purpose::STANDARD;
    let file_bytes = engine
        .decode(raw_base64)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let file_content = std::str::from_utf8(&file_bytes);
    _ = dbg!(file_content);
    Ok(String::new())
}
