use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use axum_htmx::HxBoosted;
use notify::Watcher;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = Tera::new("templates/**/*").unwrap();
}

fn render_with_context(template: &str, page_context: &Context) -> tera::Result<String> {
    let mut context = Context::new();
    context.insert("from_global_context", "value from global context");
    context.extend(page_context.clone());

    TEMPLATES.render(template, &context)
}

async fn index_page() -> Result<Html<String>, AppError> {
    let mut page_context = Context::new();
    page_context.insert("from_page_context", "value from the index page context");
    let rendered = render_with_context("pages/index.tera", &page_context)?;

    Ok(Html(rendered))
}

async fn some_page_by_id(
    HxBoosted(boosted): HxBoosted,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    let is_boosted = boosted.clone();
    let mut page_context = Context::new();
    page_context.insert("from_page_context", "value from some page context");
    page_context.insert("page_id", &id);
    page_context.insert("is_boosted", &is_boosted);
    let rendered = render_with_context("pages/some_page.tera", &page_context)?;

    Ok(Html(rendered))
}

fn setup_reload_watchers(reloader: tower_livereload::Reloader) -> Result<(), notify::Error> {
    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(
        std::path::Path::new("templates"),
        notify::RecursiveMode::Recursive,
    )?;
    watcher.watch(
        std::path::Path::new("public"),
        notify::RecursiveMode::Recursive,
    )?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let live_reload = LiveReloadLayer::new();
    let reloader = live_reload.reloader();
    setup_reload_watchers(reloader)?;

    let app = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(index_page))
        .route("/page/:id", get(some_page_by_id))
        .layer(live_reload);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
