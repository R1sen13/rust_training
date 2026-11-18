#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

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
    fn move_by(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1
        }
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
    player.move_by(Direction::Down);
    player.move_by(Direction::Right);
    player.take_damage(15);
    println!("{:?}", player);
    player.move_by(Direction::Left);
    player.move_by(Direction::Up);
    player.take_damage(50);
    println!("{:?}", player);
    let status = player.is_alive();
    println!("{}", status);
}