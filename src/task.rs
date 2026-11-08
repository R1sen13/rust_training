#[derive(Copy, Clone)]
pub enum TaskStatus {
    Completed,
    Active,
}

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