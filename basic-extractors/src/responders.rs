use axum::{http::StatusCode, response::{Html, IntoResponse}, Json};
use serde_json::{json, Value};


/// Default reply (Route: "/"), returns a str
pub async fn index() -> &'static str {
    "All good"
}

/// HTML reply (Route: "/html"), returns an HTML string
/// The HTML is then rendered by the browser
pub async fn html_index() -> Html<&'static str> {
    Html(
        "<h1><center>
        All good
        </center></h1>",
    )
}

/// JSON reply (Route: "/json"), returns a JSON object, type serde_json::Value
/// The JSON is handled by a web browser to show what data was returned
/// Other utilities may handle it however JSON is handled
pub async fn json_index() -> Json<Value> {
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
