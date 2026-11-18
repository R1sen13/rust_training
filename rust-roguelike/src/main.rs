#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Player {
    position: Position,
    health: u32,
}

fn main() {
    let player = Player {
        position: Position { x: 0, y: 0 },
        health: 20,
    };

    println!("{:?}", player);
}