struct Task {
    title: String,
    completed: bool,
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
        println!("Задача: '{}'", task.title);
        println!("Выполнена: {}", task.completed);
        println!()
    }
}
