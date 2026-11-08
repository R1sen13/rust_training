struct Task {
    title: String,
    completed: bool,
}

fn print_task(task: &Task) {
    println!("Задача: '{}'", task.title);
    println!("Выполнена: {}", task.completed);
    println!();
}

fn main() {
    let tasks = vec![
        Task {
            title: String::from("Тасочка"),
            completed: false,
        },
        Task {
            title: String::from("Йоу"),
            completed: true,
        }
    ];

    for task in tasks {
        print_task(&task);
    }
}
