use crate::StatusCode;
use axum::handler::Handler;
use axum::http::Request;
use axum::response::{Html, IntoResponse, Response};
use axum::BoxError;
use finch_entities::sea_orm::DbErr;
use tera::{Context, Tera};
use thiserror::Error;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[error("Application error: {msg}\nStatus code: {code}")]
pub struct Error {
    msg: String,
    code: StatusCode,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut ctx = Context::new();
        ctx.insert("message", &self.msg);
        ctx.insert("code", &self.code.as_str());

        let body = Tera::one_off(
            "\
    <link
      href=\"//fonts.googleapis.com/css?family=Raleway:400,300,600\"
                                 rel=\"stylesheet\"
        type=\"text/css\"
            />
            <link rel=\"stylesheet\" href=\"/static/css/normalize.css\" />
            <link rel=\"stylesheet\" href=\"/static/css/skeleton.css\" />
            <link rel=\"stylesheet\" href=\"/static/css/style.css\" />
<div class=\"container\">
Uh oh! An error ocurred. Please report this to the system administrator.
<pre>{{ code }}</pre>
<pre>{{ message }}</pre>
</div>
        ",
            &ctx,
            true,
        )
        .expect("Could not create error template while handling another error.");

        Html(body).into_response()
    }
}

impl From<migration::DbErr> for Error {
    fn from(e: DbErr) -> Self {
        error!("Database error: {}", e);

        let msg = match &e {
            DbErr::Exec(s) => format!("Execution error: {}", s),
            DbErr::Conn(s) => format!("Connection error: {}", s),
            DbErr::Json(s) => format!("JSON error: {}", s),
            DbErr::Query(s) => format!("Query error: {}", s),
            DbErr::RecordNotFound(s) => format!("Record not found error: {}", s),
            DbErr::Type(s) => format!("Type error: {}", s),
            DbErr::Custom(s) => format!("Custom error: {}", s),
        };

        Self {
            msg,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        error!("Templating error: {}", e);

        Self {
            msg: e.to_string(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type HtmlResult<T> = Result<Html<T>>;
