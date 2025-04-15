#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    // Initializes a new WorkEnvironment with no workers
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Adds a worker to the top (start) of the list
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role,
            name,
            next: self.grade.take(), // take replaces self.grade with None and gives us the old value
        });
        self.grade = Some(new_worker);
    }

    // Removes the last added worker (from the top of the list)
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(mut top_worker) = self.grade.take() {
            self.grade = top_worker.next.take(); // promote the next worker
            Some(top_worker.name)
        } else {
            None
        }
    }

    // Returns the name and role of the most recently added worker
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|w| (w.name.clone(), w.role.clone()))
    }
}
