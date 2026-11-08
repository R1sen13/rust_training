struct Task {
    title: String,
    completed: bool,
}

fn main() {
    let task = Task {
        title: String::from("Задача 1"),
        completed: false,
    };

    println!("Задача: '{}'", task.title);
    println!("Выполнена: {}", task.completed);
}
