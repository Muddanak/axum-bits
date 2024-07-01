#[derive(Clone)]
pub(crate) struct StateInfo {
    pub name: String,
    pub version: String,
    pub secret_number: u8,
    pub vec_list: Vec<u8>,
}

impl Default for StateInfo {
    fn default() -> Self {
        Self {
            name: String::from("basic-state example"),
            version: String::from("v1.3.3.7"),
            secret_number: 42,
            vec_list: vec![72, 101, 108, 108, 111],
        }
    }
}
