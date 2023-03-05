use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

use axum::{routing::get, Router, Server};
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        sys: Arc::new(Mutex::new(System::new())),
    };
    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.js", get(index_js_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(state);
    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on http://{}", addr);

    server.await.expect("Failed to start server");
}
async fn root_get() -> Html<String> {
    let html = tokio::fs::read_to_string("static/index.html")
        .await
        .unwrap();
    Html(html)
}

async fn index_js_get() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("static/index.js").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}

async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    let body: BTreeMap<usize, f32> = sys
        .cpus()
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.cpu_usage()))
        .collect();

    (StatusCode::OK, Json(body))
}
