pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    // Constructor
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list }
    }

    // Add a new Rc<String> to the vector
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    // Remove all references that point to the exact same Rc (not just equal content)
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list
            .retain(|x| !Rc::ptr_eq(x, &element));
    }
}

// Return how many strong references exist to the Rc
pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
