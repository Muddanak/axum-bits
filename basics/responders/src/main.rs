use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webserver = Router::new()

        .route("/", get(index)
                    .post(post_index))

        .route("/html", get(html_index))
        .route("/json", get(json_index))

        .fallback(error_404);

    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}

/// Default reply (Route: "/") via GET, returns a str
async fn index() -> &'static str {
    "All good"
}

/// Default reply (Route: "/") via POST
async fn post_index() -> &'static str {
    "All good (post)"
}

/// HTML reply (Route: "/html") via GET, returns an HTML string
/// The HTML is then rendered by the browser
async fn html_index() -> Html<&'static str> {
    Html(
        "<h1><center>
        All good
        </center></h1>",
    )
}

/// JSON reply (Route: "/json") via GET, returns a JSON object, type serde_json::Value
/// The JSON is handled by a web browser to show what data was returned
/// Other utilities may handle it however JSON is handled
async fn json_index() -> Json<Value> {
    Json(json!(
        {
        "data": "All good"
        }
    ))
}

/// 404 reply (Route: "*" except listed above)
/// Returns a Status Code of 404 and an HTML page upon encountering a 404 error
pub async fn error_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html("<h1><center>404 Not Found</center></h1>"),
    )
}
