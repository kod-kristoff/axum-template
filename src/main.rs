#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    // tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(axum_template::app().into_make_service())
        .await
        .unwrap();
}
