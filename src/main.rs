use blockchain_otp::create_app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = create_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Blockchain OTP service listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}