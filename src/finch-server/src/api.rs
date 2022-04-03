use axum::extract::Extension;
use axum::response::Html;
use finch_entities::sea_orm::DatabaseConnection;
use tera::Tera;
use tracing::{error, instrument};
use finch_entities::team;
use finch_entities::team::Entity as Team;
use finch_entities::sea_orm::{EntityTrait, QueryOrder, PaginatorTrait};
use crate::{HtmlResult, StatusCode};

#[instrument]
pub async fn list_teams(Extension(conn): Extension<DatabaseConnection>, Extension(templates): Extension<Tera>) -> HtmlResult<String> {
    let paginator = Team::find()
        .order_by_asc(team::Column::Name)
        .paginate(&conn, 5);

    let teams = paginator.fetch_page(1).await?;

    let mut ctx = tera::Context::new();
    ctx.insert("teams", &teams);

    let body = templates.render("api/list_teams.tera", &ctx)?;

    Ok(Html(body))
}
