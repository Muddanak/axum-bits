/// Building off of the basic-responders
/// This will introduce the basic use of extractors
use axum::{routing::get, Router};
use responders::*;

mod responders;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webserver = Router::new()
        //Basic responders
        .route("/", get(index))
        .route("/html", get(html_index))
        .route("/json", get(json_index))
        //Global Error 404
        .fallback(error_404);

    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}
