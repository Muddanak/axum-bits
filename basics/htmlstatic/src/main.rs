/// Exhibiting a basic method to handle a simple server implementation of Axum
/// This implementation introduces tower_http to provide the ServeFile service
use axum::{routing::get, Router};

// Much appreciation to mo8it@fosstodon.org for recommending this one

// Statically load the index.html file from disk and embed into the server
static INDEX: &[u8] = include_bytes!("../pages/index.html");

// Statically load the 404 file from disk and embed into the server
static NOTFOUND: &[u8] = include_bytes!("../pages/404.html");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic webserver with one default route via GET request
    // Contains a global fallback for a 404 error if the route isn't found
    let webserver = Router::new()
        // Basic routing service that returns a static html file
        .route("/", get(INDEX))
        // Global static 404 route
        .fallback(get(NOTFOUND));

    // Establish a listening socket
    // Gracefully errors out if it can't do so
    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    // Executes the server over the listening socket
    // Gracefully errors out if there are any problems
    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}
