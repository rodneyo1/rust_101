pub mod messenger;
pub use messenger::{Tracker, Logger};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Worker {
    pub track_value: Rc<String>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Worker {
        Worker {
            track_value: Rc::new("value".repeat(value)),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(format!("Warning: {}", msg));
    }

    fn info(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(format!("Info: {}", msg));
    }

    fn error(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(format!("Error: {}", msg));
    }
}
