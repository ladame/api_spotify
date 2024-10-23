use axum::{
    extract::{Json, Query},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;

#[derive(Deserialize)]
struct SearchAlbum{
    album:String,
}
async fn album(Query(params): Query<SearchAlbum>) -> impl IntoResponse{
    format!("Search album: {}", params.album)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/album", get(album));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
