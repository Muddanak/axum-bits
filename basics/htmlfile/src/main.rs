/// Exhibiting a basic method to handle a simple server implementation of Axum
/// This implementation introduces tower_http to provide the ServeFile service
use axum::Router;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic webserver with one default route via GET request
    // Contains a global fallback for a 404 error if the route isn't found
    let webserver = Router::new()
        // Basic routing service that returns a stored html file
        .route_service("/", ServeFile::new("basics/htmlfile/pages/index.html"))
        // Global 404 route
        .fallback_service(ServeFile::new("basics/htmlfile/pages/404.html"));

    // Establish a listening socket
    // Gracefully errors out if it can't do so
    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    // Executes the server over the listening socket
    // Gracefully errors out if there are any problems
    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}
