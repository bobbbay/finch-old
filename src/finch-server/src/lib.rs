#![forbid(unsafe_code)]

pub mod config;

use std::net::SocketAddr;
use std::path::PathBuf;
use axum::{Router, Server};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, get_service};
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::{info, instrument, trace};
use finch_entities::sea_orm::Database;
use migration::{Migrator, MigratorTrait};
use crate::config::Config;

/// Given a root directory, create a `PathBuf` to the templates directory.
fn template_dir(dir: &PathBuf) -> PathBuf {
    let mut template_dir = dir.clone();
    template_dir.push("templates/**/*");
    template_dir
}

/// Given a root directory, create a `PathBuf` to the static directory.
fn static_dir(dir: &PathBuf) -> PathBuf {
    let mut static_dir = dir.clone();
    static_dir.push("static");
    static_dir
}

/// Start the server. Must be run inside of an existing Tokio runtime, as well as an initialized
/// eyre/tracing-subscriber instance.
#[instrument]
pub async fn start(dir: PathBuf, config: Config) -> color_eyre::Result<()> {
    let conn = Database::connect(config.db_url).await?;
    let templates = Tera::new(template_dir(&dir).to_str().unwrap_or(""))?;

    Migrator::up(&conn, None).await?;
    trace!("Succesfully ran migrations.");

    let app = Router::new()
        .route("/", get(root))
        .nest(
            "/static",
            get_service(ServeDir::new(static_dir(&dir).to_str().unwrap_or("")))
                .handle_error(|error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Static service error: {}", error),
                    )
                }),
        )
        .layer(ServiceBuilder::new()
            .layer(Extension(conn))
            .layer(Extension(templates)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening at: {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[instrument]
async fn root(Extension(ref mut templates): Extension<Tera>) -> Result<Html<String>, (StatusCode, &'static str)> {
    #[cfg(debug_assertions)]
    templates.full_reload().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template reload error."))?;

    let ctx = tera::Context::new();
    let body = templates
        .render("index.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error."))?;
    Ok(Html(body))
}
