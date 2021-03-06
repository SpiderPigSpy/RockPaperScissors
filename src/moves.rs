use ::{Player, RED, BLUE};

pub static ALL_DIRECTIONS: &'static [Direction; 8] = &[
            Direction::Forward,
            Direction::Back,
            Direction::Left,
            Direction::Right,
            Direction::ForwardLeft,
            Direction::ForwardRight,
            Direction::BackLeft,
            Direction::BackRight,
        ];

#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum Direction {
    Forward,
    Back,
    Left,
    Right,
    ForwardLeft,
    ForwardRight,
    BackLeft,
    BackRight,
}

impl Direction {
    
    pub fn apply(&self, player: Player, from: (usize, usize)) -> (usize, usize) {
        let (x, y) = from;
        let (x, y) = (x as i32, y as i32);
        
        let (dx, dy) = match player {
            RED => { (1, 1) },
            BLUE => { (1, -1) },
        };
        
        let (to_x, to_y) = match *self {
            Direction::Forward => (x, y + dy),
            Direction::Back => (x, y - dy),
            Direction::Left => (x - dx, y),
            Direction::Right => (x + dx, y),
            Direction::ForwardLeft => (x - dx, y + dy),
            Direction::ForwardRight => (x + dx, y + dy),
            Direction::BackLeft => (x - dx, y - dy),
            Direction::BackRight => (x + dx, y - dy),
        };
        
        (to_x as usize, to_y as usize)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Move {
    pub from: (usize, usize),
    pub direction: Direction,
}

impl Move {
    pub fn new(x: usize, y: usize, direction: Direction) -> Move {
        Move {
            from: (x, y),
            direction: direction,
        }
    }
    
    pub fn apply(&self, player: Player) -> (usize, usize) {
        self.direction.apply(player, self.from)
    }
}

pub fn is_valid (movement: Move) -> bool {
    movement.direction == Direction::Forward ||
    movement.direction == Direction::ForwardLeft ||
    movement.direction == Direction::ForwardRight
}
