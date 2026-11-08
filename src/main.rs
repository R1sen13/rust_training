mod task;
mod manager;

use manager::TaskManager;

fn main() {
    let mut manager = TaskManager::new();

    manager.add_task(String::from("Rust"));
    manager.add_task(String::from("React"));

    for task in manager.tasks() {
        println!("{}: {}", task.id, task.title);
    }
}
