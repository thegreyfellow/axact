use axum::extract::State;
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
    let router = Router::new().route("/", get(root_get)).with_state(state);
    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on http://{}", addr);

    server.await.expect("Failed to start server");
}

async fn root_get(State(state): State<AppState>) -> String {
    use std::fmt::Write;

    let mut cpus = String::new();
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;
        writeln!(cpus, "CPU {}: {}%", i, cpu.cpu_usage()).unwrap();
    }

    cpus
}
