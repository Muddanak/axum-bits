use serde::Deserialize;

/// The Student Struct
#[derive(Clone, Default, Deserialize)]
pub struct Student {
    pub name: String,
    pub grade: usize,
}