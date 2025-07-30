use std::net::SocketAddr;
use std::path::Path;

use axum::routing::post;
use axum::{routing::get, Router};
use axum::response::Html;
use serde::Serialize;

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_text() -> Html<String> {
    //Html("<h1>Hello, World!</h1>")

    //const HTML: &str = include_str!("hello.html");
    //Html(HTML)

    let path = Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: "Hola Mundo".to_string(),
    })
}

async fn post_html() -> Html<String> {
    let path = Path::new("src/post.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn say_hello_post() -> Html<String> {
    Html("Hello from POST".to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/json", get(say_hello_json))
        .route("/hello_post", get(post_html))
        .route("/post", post(say_hello_post));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
