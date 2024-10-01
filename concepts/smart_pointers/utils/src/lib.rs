use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Tool {
    pub name: String,
    pub owner: Rc<Owner>,
}

pub struct Owner{
    pub name: String,
    pub tools: RefCell<Vec<Weak<Tool>>>,
}

impl Owner {
    pub fn new(name: String) -> Owner {
        Owner {
            name,
            tools: RefCell::new(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
}
