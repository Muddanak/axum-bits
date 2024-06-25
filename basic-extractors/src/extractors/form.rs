use axum::Form;

use super::extract_struct::Student;

/// Extract from the URL the given Form data in a POST request and have it deserialize into the Student struct
/// Requires:
///     name: String
///     grade: usize
/// Example: 127.0.0.1:8080/studentform
///     POST data: "name=Bob McBobson&grade=85"
pub(crate) async fn get_student_info_form(Form(studentinfo): Form<Student>) -> String {
    format!(
        "Name: {} | Grade: {}",
        studentinfo.name, studentinfo.grade
    )
}