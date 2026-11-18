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

impl Player {
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    fn take_damage(&mut self, damage: u32) {
        self.health = if damage >= self.health { 0 } else { self.health - damage };
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn main() {
    let mut player = Player {
        position: Position { x: 0, y: 0 },
        health: 20,
    };
    println!("{:?}", player);
    player.move_by(1, 5);
    player.take_damage(15);
    println!("{:?}", player);
    player.move_by(-2, -10);
    player.take_damage(50);
    println!("{:?}", player);
    let status = player.is_alive();
    println!("{}", status);
}