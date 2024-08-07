/// Exhibiting a basic method to handle a simple server implementation of Axum
use axum::{http::StatusCode, routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic webserver with one default route via GET request
    // Contains a global fallback for a 404 error if the route isn't found
    let webserver = Router::new()
        // Basic route for the default index state
        // GET handler request
        .route("/", get(index_for_example))
        // Global 404 route tuple
        .fallback((StatusCode::NOT_FOUND, "404: Not Found"));

    // Establish a listening socket
    // Gracefully errors out if it can't do so
    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    // Executes the server over the listening socket
    // Gracefully errors out if there are any problems
    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}

/// The function called for the default index state
async fn index_for_example() -> &'static str {
    "All good"
}
