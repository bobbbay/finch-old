#![forbid(unsafe_code)]

pub mod config;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use axum::{Router, Server};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use tera::Tera;
use tower::ServiceBuilder;
use tracing::{info, instrument};
use crate::config::Config;

/// Start the server. Must be run inside of an existing Tokio runtime, as well as an initialized
/// eyre/tracing-subscriber instance.
#[instrument]
pub async fn start(mut dir: PathBuf, config: Config) -> color_eyre::Result<()> {
    dir.push("templates/**/*");
    let templates = Tera::new(dir.to_str().unwrap_or(""))?;

    let app = Router::new()
        .route("/", get(root))
        .layer(ServiceBuilder::new()
            .layer(Extension(templates)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening at: {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[instrument]
async fn root(Extension(ref templates): Extension<Tera>) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = templates
        .render("index.tera", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "Template error."))?;
    Ok(Html(body))
}
