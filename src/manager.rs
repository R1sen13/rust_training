use crate::task::{Task, TaskStatus};
use std::fs;
use std::path::Path;

const STORAGE_FILE: &str = "tasks.json";

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
    
    pub fn load() -> Self {
        if !Path::new(STORAGE_FILE).exists() {
            return Self::new();
        }
        
        let content = fs::read_to_string(STORAGE_FILE)
            .expect("No access to 'tasks.json'");
        
        let tasks: Vec<Task> = serde_json::from_str(&content)
            .expect("Task load error");
        
        let next_id = tasks
            .iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0) + 1;
        
        Self { tasks, next_id }
    }
    
    pub fn save(&self) {
        let content = serde_json::to_string_pretty(&self.tasks)
            .expect("Could not serialize tasks to json");
        
        fs::write(STORAGE_FILE, content)
            .expect("No access to 'tasks.json'")
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

    pub fn complete_task(&mut self, id: usize) -> bool {
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.change_status();
                return true;
            }
        }
        false
    }

    pub fn remove_task(&mut self, id: usize) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }
    
    pub fn print_tasks(&self) {
        for task in self.tasks.iter() {
            println!("Task {}: {}", task.id, task.title);
            match task.status {
                TaskStatus::Active => println!("Status: Active"),
                TaskStatus::Completed => println!("Status: Completed"),
            }
            println!()
        }
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}