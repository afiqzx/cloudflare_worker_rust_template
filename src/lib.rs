#[macro_use]
mod internal_macro;

mod app_state;
mod handlers;

use std::collections::HashMap;
use std::sync::Arc;

use app_state::{serve_static_files, AppState, StaticFile, StaticFileType};
use axum::{routing::get, Router};
use handlers::{about, root};
use minijinja::Environment;
use tower_service::Service;
use worker::*;

fn router() -> Router {
    let mut env = Environment::new();
    include_all_files![ env;
    "index.html"; "../templates/index.html",
    "about.html"; "../templates/about.html",
    ];

    let mut static_files = HashMap::<&'static str, StaticFile>::new();
    static_files.insert(
        "output.css",
        StaticFile {
            ftype: StaticFileType::CSS,
            content: include_str!("../css/output.css"),
        },
    );

    let app_state = AppState {
        jinja_env: env,
        static_files: Arc::new(static_files.clone()),
    };

    Router::new()
        .route("/", get(root))
        .route("/about", get(about))
        .route(
            "/static/:file_name",
            get(serve_static_files).with_state(static_files),
        )
        .with_state(app_state)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}
