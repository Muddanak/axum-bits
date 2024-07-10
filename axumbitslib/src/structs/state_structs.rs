use std::sync::{Arc, Mutex};

/// The StateInfo struct.
#[derive(Clone)]
pub struct StateInfo {
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

/// The mutable ArcMutexStateInfo struct.
#[derive(Clone)]
pub struct ArcMutexStateInfo {
    pub name: Arc<Mutex<String>>,
    pub version: String,
    pub secret_number: Arc<Mutex<u8>>,
    pub vec_list: Arc<Mutex<Vec<u8>>>,
}

impl Default for ArcMutexStateInfo {
    fn default() -> Self {
        Self {
            name: Arc::new(Mutex::new(String::from("basic-state example"))),
            version: String::from("v1.3.3.7"),
            secret_number: Arc::new(Mutex::new(42)),
            vec_list: Arc::new(Mutex::new(vec![72, 101, 108, 108, 111])),
        }
    }
}
