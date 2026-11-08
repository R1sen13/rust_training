use crate::task::{Task, TaskStatus};

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_task(&mut self, title: String) {
        let task = Task {
            id: self.next_id,
            title,
            status: TaskStatus::Active,
        };
        
        self.tasks.push(task);
        self.next_id += 1;
    }
    
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}