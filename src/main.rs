#[derive(Copy, Clone)]
enum TaskStatus {
    Completed,
    Active,
}

struct Task {
    title: String,
    status: TaskStatus,
}

fn print_task(task: &Task) {
    println!("Задача: '{}'", task.title);
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
            TaskStatus::Active => {},
        }
    }
    count
}

fn main() {
    let tasks = vec![
        Task {
            title: String::from("Тасочка"),
            status: TaskStatus::Completed,
        },
        Task {
            title: String::from("Йоу"),
            status: TaskStatus::Active,
        },
    ];

    for task in &tasks {
        print_task(task);
    }
    let completed = count_completed(&tasks);
    println!("Completed: {}", completed);
}
