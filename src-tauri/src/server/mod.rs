use anyhow::Context;
use axum::{routing::get, Router};
use tauri::AppHandle;
use tokio::{net::TcpListener, signal};

#[tokio::main]
pub async fn init(app: AppHandle) -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4875")
        .await
        .context("could not bind to endpoint")?;

    let router = Router::new()
        .route("/authorize", get(get_info))
        .with_state(app.clone());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("error while starting API server")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn get_info() {}
