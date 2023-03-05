use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_get));
    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on http://{}", addr);

    server.await.expect("Failed to start server");
}

async fn root_get() -> &'static str {
    println!("Hello, I'm root!");
    "Hello, I'm root!"
}
