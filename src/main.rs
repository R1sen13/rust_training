struct Task {
    title: String,
    completed: bool,
}

fn print_task(task: &Task) {
    println!("Задача: '{}'", task.title);
    println!("Выполнена: {}", task.completed);
    println!();
}

fn count_completed(tasks: &[Task]) -> usize {
    let mut count = 0;
    for task in tasks {
        if task.completed {
            count += 1;
        }
    }
    count
}

fn main() {
    let tasks = vec![
        Task {
            title: String::from("Тасочка"),
            completed: true,
        },
        Task {
            title: String::from("Йоу"),
            completed: true,
        },
    ];

    for task in &tasks {
        print_task(task);
    }
    let completed = count_completed(&tasks);
    println!("Completed: {}", completed);
}
