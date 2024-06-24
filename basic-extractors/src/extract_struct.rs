use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
pub struct Student {
    pub name: String,
    pub grade: usize,
}
