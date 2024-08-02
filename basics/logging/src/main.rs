/// Exhibiting a basic method to handle a simple server implementation of Axum
use axum::Router;
use tower_http::{services::ServeFile, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Axum, working with tower_http, needs a tracing_subscriber to listen
    // for events that are emitted
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                // Tell tower_http to set the tracing/logging level to debug to capture almost all events
                // Understandably, this is quite noisy but full of information
                .unwrap_or_else(|_| "tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Basic webserver with one default route via GET request
    // Contains a global fallback for a 404 error if the route isn't found
    let webserver = Router::new()
        // Basic route for the default index state
        // GET handler request
        .route_service("/", ServeFile::new("basics/logging/pages/index.html"))
        // Global 404 route response tuple
        //.fallback((StatusCode::NOT_FOUND, "Not found"))
        .fallback_service(ServeFile::new("basics/logging/pages/notfound.html"))
        // Set the specific layer for tracing/logging from tower_http
        .layer(TraceLayer::new_for_http());

    // Establish a listening socket
    // Gracefully errors out if it can't do so
    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    //println!("Starting server on {} port {}", weblistener.)

    // Executes the server over the listening socket
    // Gracefully errors out if there are any problems
    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}

/* /// The function called for the default index state
async fn index_for_example() -> &'static str {
    "All good"
} */
