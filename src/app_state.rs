use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use minijinja::Environment;

#[derive(Clone)]
#[allow(dead_code, clippy::upper_case_acronyms)]
pub(crate) enum StaticFileType {
    CSS,
    JS,
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct StaticFile {
    pub(crate) ftype: StaticFileType,
    pub(crate) content: &'static str,
}

impl StaticFile {
    fn generate_response(&self) -> Response {
        let ftype = match self.ftype {
            StaticFileType::CSS => "text/css",
            StaticFileType::JS => "text/javascript",
        };

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", ftype)
            .header("Cache-Control", "max-age=86400")
            .body(Body::from(self.content))
            .unwrap()
    }
}

pub(crate) async fn serve_static_files(
    Path(file_name): Path<String>,
    static_files: State<StaticFiles>,
) -> Response {
    match static_files.get(AsRef::<str>::as_ref(&file_name)) {
        Some(file) => file.generate_response(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap(),
    }
}

pub(crate) type StaticFiles = HashMap<&'static str, StaticFile>;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) jinja_env: Environment<'static>,
    pub(crate) static_files: Arc<StaticFiles>,
}

impl<'a> FromRef<AppState> for Environment<'a> {
    fn from_ref(input: &AppState) -> Self {
        input.jinja_env.clone()
    }
}

impl FromRef<AppState> for Arc<StaticFiles> {
    fn from_ref(input: &AppState) -> Self {
        input.static_files.clone()
    }
}
