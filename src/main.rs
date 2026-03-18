use axum::{
    Json, Router,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::get,
};

use crate::setup::AppConfig;

pub mod common;
pub mod server_config;
pub mod setup;

mod ffmpeg;
mod hls;

#[tokio::main]
async fn main() {
    let config = AppConfig::load().await;
    let _ = hls::m3u8::generate_hls_response_m3u8_v3(3, 1, 3.0, 1);
    let app = Router::new().route("/m3u8", get(build_m3u8));

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
            .await
            .expect("Can't run app on port");

    axum::serve(listener, app).await;
}
async fn build_m3u8() -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    header_map.insert(
        header::CONTENT_TYPE,
        "application/vnd.apple.mpegurl".parse().unwrap(),
    );
    let data = hls::m3u8::read_m3u8();

    return (StatusCode::OK, header_map, data);
}
