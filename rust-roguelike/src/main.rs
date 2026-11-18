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

#[derive(Debug)]
struct Map {
    width: u32,
    height: u32,
}

impl Map {
    fn contains(&self, position: &Position) -> bool {
        !(position.x >= self.width || position.x < 0 || position.y >= self.height || position.y < 0)
    }
}

impl Player {
    fn move_in(&mut self, dx: i32, dy: i32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    fn move_by(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1
        }
    }

    fn try_move(&mut self, direction: Direction, map: &Map) -> bool {
        let current_position = self.position;

        match direction {
            Direction::Up => current_position.y += 1,
            Direction::Down => current_position.y -= 1,
            Direction::Left => current_position.x -= 1,
            Direction::Right => current_position.x += 1
        }

        if (!map.contains(&current_position)) {
            return false;
        }

        self.move_by(direction);
        
        return true;
    }

    fn take_damage(&mut self, damage: u32) {
        self.health = if damage >= self.health { 0 } else { self.health - damage };
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn change_position(&mut self, position: &Position) {
        self.position.x = position.x;
        self.position.y = position.y;
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