use ::{Field, WIDTH, HEIGHT, RED, BLUE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl Move {
    pub fn is_same_from_to(&self) -> bool {
        self.from == self.to
    }
    
    pub fn is_vertical(&self) -> bool {
        self.from.0 == self.to.0
    }
    
    pub fn is_horizontal(&self) -> bool {
        self.from.1 == self.to.1
    }
    
    pub fn is_one_cell_move(&self) -> bool {
        (self.from.0 as i32 - self.to.0 as i32).abs() <= 1 &&
        (self.from.1 as i32 - self.to.1 as i32).abs() <= 1
    }
    
    pub fn is_up(&self) -> bool {
        self.to.1 as i32 - self.from.1 as i32 > 0
    }
}

pub trait MoveCondition {
    fn available (&self, field: &Field, movement: Move) -> bool;
}

pub struct OnlyForwardMove;

impl MoveCondition for OnlyForwardMove {
    fn available (&self, field: &Field, movement: Move) -> bool {
        if movement.is_same_from_to()
           || !movement.is_vertical() 
           || !movement.is_one_cell_move() { return false; }
           
        let (from_x, from_y) = movement.from;
        let (to_x, to_y) = movement.to;
        
        if from_x >= WIDTH || to_x >= WIDTH
           || from_y >= HEIGHT || to_y >= HEIGHT { return false; }
           
        if let Some(unit) = field.rows[from_x][from_y] {
            let up = movement.is_up();
            match (unit.owner, up) {
                (RED, false) | (BLUE, true) => { return false; },
                _ => {}
            }
            
            if (up && to_y == HEIGHT - 1) || (!up && to_y == 0) { return false; }
        
        } else {
            return false;
        }
        true
    }
}
