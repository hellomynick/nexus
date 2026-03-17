use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

pub mod common;
mod hls;
pub mod server_config;

const PORT: &str = "9320";

#[tokio::main]
async fn main() {
    let text = format!("{:?}", String::from_utf8(data).unwrap());
    print!("{}", text);

    let app = Router::new().route("/m3u8", get(build_m3u8));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .expect("Can't run app on port");
}

fn build_m3u8() -> IntoResponse {
    let _ = hls::m3u8::generate_hls_response_m3u8_v3(3, 1, 3.0, 1);
    let data = hls::m3u8::read_m3u8();
}
