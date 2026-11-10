use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Completed,
    Active,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn change_status(&mut self) {
        match self.status {
            TaskStatus::Completed => self.status = TaskStatus::Active,
            TaskStatus::Active => self.status = TaskStatus::Completed,
        }
    }
}