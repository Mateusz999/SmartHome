use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[derive(Serialize)]
struct SensorData {
    temperature: f32,
    humidity: u8,
}

async fn get_data() -> Json<SensorData> {
    Json(SensorData {
        temperature: 28.5,
        humidity: 45,
    })
}
//
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/data", get(get_data))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("API działa na http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service())
        .await
        .unwrap();
}
