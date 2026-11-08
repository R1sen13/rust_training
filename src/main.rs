#[derive(Copy, Clone)]
enum TaskStatus {
    Completed,
    Active,
}

struct Task {
    id: usize,
    title: String,
    status: TaskStatus,
}

impl Task {
    fn change_status(&mut self) {
        match self.status {
            TaskStatus::Completed => self.status = TaskStatus::Active,
            TaskStatus::Active => self.status = TaskStatus::Completed,
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, title: String) {
    let new_task = Task {
        id: tasks.len() + 1,
        title,
        status: TaskStatus::Active,
    };
    tasks.push(new_task);
}

fn print_task(task: &Task) {
    println!("Задача {}: '{}'", task.id, task.title);
    match task.status {
        TaskStatus::Active => println!("Статус: Активна"),
        TaskStatus::Completed => println!("Статус: Выполнена"),
    }
    println!();
}

fn count_completed(tasks: &[Task]) -> usize {
    let mut count = 0;
    for task in tasks {
        match task.status {
            TaskStatus::Completed => count += 1,
            TaskStatus::Active => {}
        }
    }
    count
}

fn main() {
    let mut tasks = vec![
        Task {
            id: 1,
            title: String::from("Learning Rust"),
            status: TaskStatus::Completed,
        },
        Task {
            id: 2,
            title: String::from("Making Web App"),
            status: TaskStatus::Active,
        },
    ];
    if let Some(second_task) = tasks.get_mut(1) {
        second_task.change_status();
    } else {
        println!("Task was not found");
    }
    add_task(&mut tasks, String::from("Getting smarter"));

    for task in &tasks {
        print_task(task);
    }
    let completed = count_completed(&tasks);
    println!("Completed: {}", completed);
}
