use std::sync::{Arc, Mutex, Weak};

pub struct Tool {
    pub name: String,
    pub owner: Arc<Owner>,
}

pub struct Owner{
    pub name: String,
    pub tools: Arc<Mutex<Vec<Weak<Tool>>>>,
}

impl Owner {
    pub fn new(name: String) -> Owner {
        Owner {
            name,
            tools: Arc::new(Mutex::new(vec![])),
        }
    }
}