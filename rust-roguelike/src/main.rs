#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Wait,
    Move(Direction),
    Quit
}

#[derive(Debug, Clone, Copy)]
enum DamageResult {
    Damaged { previous: u32, current: u32 },
    Died,
}

#[derive(Debug, Clone, Copy)]
enum MoveResult {
    Moved(Position),
    Blocked(Position),
}

#[derive(Debug, Clone, Copy)]
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
        !(position.x >= self.width as i32 || position.x < 0 || position.y >= self.height as i32 || position.y < 0)
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

    fn try_move(&mut self, direction: Direction, map: &Map) -> MoveResult {
        let mut current_position = self.position;

        match direction {
            Direction::Up => current_position.y += 1,
            Direction::Down => current_position.y -= 1,
            Direction::Left => current_position.x -= 1,
            Direction::Right => current_position.x += 1
        }

        if (!map.contains(&current_position)) {
            return  MoveResult::Blocked(current_position);
        }

        self.move_by(direction);

        return MoveResult::Moved(current_position);
    }

    fn take_damage(&mut self, damage: u32) -> DamageResult {
        return if self.health == 0 {
            DamageResult::Died
        } else {
            let previous = self.health;
            self.health = if damage >= self.health { 0 } else { self.health - damage };
            let current = self.health;
            return if current == 0 {
                DamageResult::Died
            } else {
                DamageResult::Damaged { previous, current }
            }
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn change_position(&mut self, position: &Position) {
        self.position.x = position.x;
        self.position.y = position.y;
    }

    fn apply_action(&mut self, action: Action, map: &Map) {
        match action {
            Action::Wait => println!("Health: {}\nYou are waiting", self.health),
            Action::Move(direction) => { let position = self.try_move(direction, map); println!("Moved: {:?}", position) },
            Action::Quit => { println!("Quiting..."); std::process::exit(0); },
        }
    }
}

fn main() {
    let mut player = Player {
        position: Position { x: 0, y: 0 },
        health: 20,
    };

    let mut map = Map {
        width: 800,
        height: 600,
    };

    player.apply_action(Action::Move(Direction::Up), &map);
    player.apply_action(Action::Wait, &map);
    player.apply_action(Action::Quit, &map);
}