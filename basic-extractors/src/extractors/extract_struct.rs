use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
pub(crate) struct Student {
    pub name: String,
    pub grade: usize,
}
