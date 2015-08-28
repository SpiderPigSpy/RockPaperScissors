extern crate rand;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
pub const ROWS: usize = 2;

const RED: Player = Player::Red;
const BLUE: Player = Player::Blue;

const ROCK: RPS = RPS::Rock;
const PAPER: RPS = RPS::Paper;
const SCISSORS: RPS = RPS::Scissors;

const WIN: Outcome = Outcome::Win;
const LOSE: Outcome = Outcome::Lose;
const DRAW: Outcome = Outcome::Draw;

pub mod move_conditions;
pub mod win_conditions;
mod field;

use move_conditions::{MoveCondition, Move};
use win_conditions::{WinCondition};
pub use field::Field;

#[derive(Clone)]
pub struct Game<T: MoveCondition, E: WinCondition> {
    turns: u32,
    current_turn: Player,
    winner: Option<Player>,
    field: Field,
    rules: Rules<T, E>,
}

impl<T: MoveCondition, E: WinCondition> Game<T, E> {
    pub fn new(rules: Rules<T, E>) -> Game<T, E> {
        Game {
            turns: 1,
            current_turn: RED,
            winner: None,
            field: Field::new(),
            rules: rules,
        }
    }
    
    pub fn turns(&self) -> u32 { self.turns }
    pub fn current_turn(&self) -> Player { self.current_turn }
    pub fn winner(&self) -> Option<Player> { self.winner }
    pub fn field(&self) -> &Field { &self.field }
    
    pub fn make_move(&mut self, movement: Move) -> Result<Option<Outcome>, MoveError> {
        if self.winner.is_some() { return Err(MoveError::GameAlreadyFinished); }
        
        if !self.rules.move_condition.available(&self.field, movement) {
            return Err(MoveError::InvalidMove);
        }
        
        let (from_x, from_y) = movement.from;
        let (to_x, to_y) = movement.to;
        
        let attack_outcome = match (self.field.rows[from_x][from_y], self.field.rows[to_x][to_y]) {
            (None, _) => {
                return Err(MoveError::NonsenseMove);
            },
            (Some(unit), None) => {
                if unit.owner != self.current_turn { return Err(MoveError::WrongOwner); }
                None
            },
            (Some(attacker), Some(defender)) => {
                if attacker.owner != self.current_turn { return Err(MoveError::WrongOwner); }
                if attacker.owner == defender.owner { return Err(MoveError::SameOwner); }
                Some( attacker.attack(&defender) )
            }
        };
        
        if let Some(outcome) = attack_outcome {
            match outcome {
                WIN => {
                    self.field.rows[to_x][to_y] = self.field.rows[from_x][from_y];
                    self.field.rows[from_x][from_y] = None;
                    self.field.rows[to_x][to_y].as_mut().unwrap().visible = true;
                },
                LOSE => {
                    self.field.rows[from_x][from_y] = None;
                    self.field.rows[to_x][to_y].as_mut().unwrap().visible = true;
                },
                DRAW => {
                    self.field.rows[from_x][from_y].as_mut().unwrap().visible = true;
                    self.field.rows[to_x][to_y].as_mut().unwrap().visible = true;
                }
            }
        } else {
            self.field.rows[to_x][to_y] = self.field.rows[from_x][from_y];
            self.field.rows[from_x][from_y] = None;
        }
        
        self.winner = self.rules.win_condition.winner(&self.field);
        self.turns += 1;
        self.current_turn = self.current_turn.next();
        
        Ok(attack_outcome)
    }
}

pub enum MoveError {
    GameAlreadyFinished,
    InvalidMove,
    NonsenseMove,
    WrongOwner,
    SameOwner,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    fig: RPS,
    owner: Player,
    visible: bool,
}

impl Unit {
    fn new(fig: RPS, owner: Player,) -> Unit {
        Unit {
            fig: fig,
            owner: owner,
            visible: false,
        }
    }
    
    fn attack(&self, opponent: &Unit) -> Outcome {
        self.fig.attack(opponent.fig)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    fn next(&self) -> Player {
        match *self {
            RED => BLUE,
            BLUE => RED,
        }
    }
    
    fn unit(&self, fig: RPS) -> Unit {
        Unit::new(fig, *self)
    }
    
    fn random_unit(&self) -> Unit {
        self.unit(RPS::random())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn attack(&self, opponent: RPS) -> Outcome {
        match (*self, opponent) {
            (PAPER, ROCK) | (ROCK, SCISSORS) | (SCISSORS, PAPER) => WIN,
            (ROCK, PAPER) | (SCISSORS, ROCK) | (PAPER, SCISSORS) => LOSE,
            _ => DRAW,
        }
    }
    
    fn random() -> RPS {
        match rand::random::<usize>() % 3 {
            0 => ROCK,
            1 => PAPER,
            2 => SCISSORS,
            _ => { panic!("rand::random::<usize>() % 3 returned not 0, nor 1, nor 2"); }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Clone)]
pub struct Rules<T: MoveCondition, E: WinCondition> {
    pub move_condition: T,
    pub win_condition: E,
}
