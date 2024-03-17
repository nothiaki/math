use axum::{routing::get, Router};
mod controller;

#[tokio::main]
async fn main() {
    const PORT: u16 = 3000;
    let app = Router::new()
        .route("/sum", get(controller::arithmetic::sum))
        .route("/sub", get(controller::arithmetic::sub))
        .route("/area/parallelogram", get(controller::area::parallelogram));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:{PORT}")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap()
}
