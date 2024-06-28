use axum::Json;
use serde_json::{json, Value};

use super::extract_struct::Student;

/// Extract from the URL a JSON given set of data and have it deserialize into the Student struct
/// Requires:
///     name: String
///     grade: usize
/// Example: 127.0.0.1:8080/studentjson
/// POST data:
///     {"name":"Bob McBobson", "grade":85}
/// OUTPUT:
///     {"grade":75, "name":"Bob McBobson"}
pub(crate) async fn post_student_info_json(Json(studentinfo): Json<Student>) -> Json<Value> {
    Json(json!({ "name": studentinfo.name, 
                "grade": studentinfo.grade - 10}))
}
