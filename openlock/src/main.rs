mod templates;

use anyhow::Context;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use templates::IndexTemplate;

async fn status() -> &'static str {
    return "OK";
}

async fn index() -> Result<IndexTemplate, AppError> {
    return Ok(IndexTemplate {});
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "openlock=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_router = Router::new().route("/status", get(status));

    let pwd = std::env::current_dir().context("failed to get current directory")?;
    let static_dir = ServeDir::new(format!("{}/static", pwd.to_str().unwrap_or("/srv")));
    let assets_dir = ServeDir::new(format!("{}/assets", pwd.to_str().unwrap_or("/srv")));

    let app = Router::new()
        .route("/", get(index))
        .nest("/api", api_router)
        .nest_service("/static", static_dir)
        .nest_service("/assets", assets_dir);

    let port = 3000_u16;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("starting server on port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error while starting server")?;

    return Ok(());
}

// custom error type
struct AppError(anyhow::Error);

// helps axum to convert `AppError` into a response
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("server error: {}", self.0),
        )
            .into_response()
    }
}

// use `?` operator on functions that return `Result<_, anyhow::Error>`
// converting them to `Result<_, AppError>`
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}
