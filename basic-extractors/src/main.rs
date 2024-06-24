/// Building off of the basic-responders
/// This will introduce the basic use of extractors
use axum::{extract::{Path, Query}, response::Html, routing::get, Router};
use extract_struct::Student;
use responders::*;

mod extract_struct;
mod responders;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webserver = Router::new()
        // Basic responders
        .route("/", get(index))
        .route("/html", get(html_index))
        .route("/json", get(json_index))

        // Basic extractors
        .route("/student", get(get_student_info_query).post(get_student_info_query))     //Query
        .route("/:name/:grade", get(get_student_info_path)) //Path

        //Global Error 404
        .fallback(error_404);

    let weblistener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve::serve(weblistener, webserver).await?;

    Ok(())
}

/// Extract from the URL a Path given the data and have it deserialize into the Student struct
/// Requires:
///     name: String
///     grade: usize
/// Example: 127.0.0.1:8080/Bob McBobson/85
async fn get_student_info_path(Path(studentinfo): Path<Student>) -> Html<String> {
    let studentstring = format!(
        "<h1><center>
            Name: {} | Grade: {}
            </center></h1>",
        studentinfo.name, studentinfo.grade
    );

    Html(studentstring)
}

/// Extract from the URL a Query given the data and have it deserialize into the Student struct
/// Requires: 
///     name: String
///     grade: usize
/// Example: 127.0.0.1:8080/student?name=Bob McBobson&grade=85
async fn get_student_info_query(Query(studentinfo): Query<Student>) -> Html<String> {
    let studentstring = format!(
        "<h1><center>
            Name: {} | Grade: {}
            </center></h1>",
        studentinfo.name, studentinfo.grade
    );

    Html(studentstring)
}