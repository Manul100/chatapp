use chatapp_backend::api::router::create_router;

#[tokio::main]
async fn main() {

    let app = create_router().await;
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
