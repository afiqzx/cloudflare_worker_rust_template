use axum::extract::State;
use axum::response::Html;
use minijinja::Environment;
use serde::{Deserialize, Serialize};

pub(crate) async fn root(loader: State<Environment<'static>>) -> Html<String> {
    let tmpl = loader.get_template("index.html").unwrap();
    let respond = tmpl.render(());

    respond.unwrap().into()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RenderData {
    pub(crate) github_link: &'static str,
}

pub(crate) async fn about(loader: State<Environment<'static>>) -> Html<String> {
    let data = RenderData {
        github_link: "https://github.com/testuser",
    };
    let template = loader.get_template("about.html");
    dbg!(&template);
    let tmpl = template.unwrap();
    let respond = tmpl.render(data);

    respond.unwrap().into()
}
