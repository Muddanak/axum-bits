use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use axumbitslib::StateInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state_to_use = StateInfo::default();

    let webserver = Router::new()
        .route("/state", get(state))
        .fallback(error_404)
        .with_state(state_to_use);

    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}

/// Default reply (Route: "/") via GET, returns a str
async fn state(State(stateinfo): State<StateInfo>) -> Html<String> {
    Html(format!(
        "<h1><center>
            Name: {}<br>
            Version: {}<br>
            Secret Number: {}<br>
            Vector List as a String: {}<br>
            </center></h1>
            ",
        stateinfo.name,
        stateinfo.version,
        stateinfo.secret_number,
        stateinfo
            .vec_list
            .iter()
            .map(|&val| val as char)
            .collect::<String>()
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
