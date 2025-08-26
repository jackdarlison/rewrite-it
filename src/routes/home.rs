use askama::Template;
use axum::response::{Html, IntoResponse};



#[derive(Template)]
#[template(path = "home.html")]
struct Home {

}

pub async fn home() -> impl IntoResponse {
    let template = Home {};

    Html(template.render().expect("err"))
}