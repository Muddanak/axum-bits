use axum::{extract::Path, response::Html};

use super::extract_struct::Student;


/// Extract from the URL a Path given the data and have it deserialize into the Student struct
/// Requires:
///     name: String
///     grade: usize
/// Example: 127.0.0.1:8080/studentpath/Bob McBobson/85
pub(crate) async fn get_student_info_path(Path(studentinfo): Path<Student>) -> Html<String> {
    Html(format!(
            "<h1><center>
            Name: {} | Grade: {}
            </center></h1>",
            studentinfo.name, studentinfo.grade
        ))
}