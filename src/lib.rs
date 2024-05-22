use axum::extract::State;
use axum::response::Html;
use axum::{routing::get, Router};
use minijinja::{context, path_loader, Environment};
use serde::{Deserialize, Serialize};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    let mut env = Environment::new();
    // loader is not available for wasm target (there's no filesystem in cloudflare worker, duhhh)
    //env.set_loader(path_loader("templates"));
    _ = env
        .add_template("index.html", include_str!("../templates/index.html"))
        .unwrap();
    _ = env
        .add_template("about.html", include_str!("../templates/about.html"))
        .unwrap();

    Router::new()
        .route("/", get(root))
        .route("/about", get(about))
        .with_state(env)
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

pub async fn root(loader: State<Environment<'static>>) -> Html<String> {
    let tmpl = loader.get_template("index.html").unwrap();
    let respond = tmpl.render(());

    respond.unwrap().into()
}

#[derive(Serialize, Deserialize)]
struct RenderData {
    twitter_link: &'static str,
    github_link: &'static str,
}

pub async fn about(loader: State<Environment<'static>>) -> Html<String> {
    let data = RenderData {
        twitter_link: "https://twitter.com/afiq_xyz",
        github_link: "https://github.com/afiqzx",
    };
    let template = loader.get_template("about.html");
    dbg!(&template);
    let tmpl = template.unwrap();
    let respond = tmpl.render(data);

    respond.unwrap().into()
}
