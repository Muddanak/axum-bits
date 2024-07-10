use axum::{extract::Query, response::Html};

//use super::extract_struct::Student;
use axumbitslib::Student;

/// Extract from the URL a Query given the data and have it deserialize into the Student struct
/// Requires: 
///     name: String
///     grade: usize
/// Example: 127.0.0.1:8080/studentquery?name=Bob McBobson&grade=85
pub(crate) async fn get_student_info_query(Query(studentinfo): Query<Student>) -> Html<String> {
    Html(format!(
        "<h1><center>
        Name: {} | Grade: {}
        </center></h1>",
        studentinfo.name, studentinfo.grade
    ))
}