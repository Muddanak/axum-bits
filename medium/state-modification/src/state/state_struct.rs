use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct StateInfo {
    pub name: Arc<Mutex<String>>,
    pub version: String,
    pub secret_number: Arc<Mutex<u8>>,
    pub vec_list: Arc<Mutex<Vec<u8>>>,
}

impl Default for StateInfo {
    fn default() -> Self {
        Self {
            name: Arc::new(Mutex::new(String::from("basic-state example"))),
            version: String::from("v1.3.3.7"),
            secret_number: Arc::new(Mutex::new(42)),
            vec_list: Arc::new(Mutex::new(vec![72, 101, 108, 108, 111])),
        }
    }
}
