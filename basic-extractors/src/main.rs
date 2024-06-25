/// Building off of the basic-responders
/// This will introduce the basic use of extractors
use axum::{routing::{get, post}, Router};
use responders::*;
use extractors::{form::*, json::*, path::*, query::*};

mod responders;
mod extractors;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webserver = Router::new()
        // Basic responders
        .route("/", get(index))
        .route("/html", get(html_index))
        .route("/json", get(json_index))

        // Basic extractors
        // Please see the individual implementations of each extractor in the extractors folder
        .route("/studentquery", get(get_student_info_query))     //Query
        .route("/studentpath/:name/:grade", get(get_student_info_path)) //Path
        .route("/studentform", post(get_student_info_form))     //Form
        .route("/studentjson", post(get_student_info_json))     //JSON

        //Global Error 404
        .fallback(error_404);

    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}