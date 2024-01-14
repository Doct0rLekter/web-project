use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let address = SocketAddr::new([0,0,0,0].into(), 3000);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
