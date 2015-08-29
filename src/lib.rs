extern crate rand;

/// Width of game board
pub const WIDTH: usize = 8;
/// Height of game board
pub const HEIGHT: usize = 8;
/// Number of rows
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
pub mod unit;
pub mod field;

use move_conditions::{MoveCondition, Move};
use win_conditions::{WinCondition};
use field::{Field, PovField};
use unit::{Unit, GeneralUnit};

use std::marker::PhantomData;

#[derive(Clone)]
pub struct Game<T: MoveCondition<GeneralUnit>, E: WinCondition<GeneralUnit>> {
    turns: u32,
    current_turn: Player,
    winner: Option<Player>,
    field: Field<GeneralUnit>,
    rules: Rules<GeneralUnit, T, E>,
}

impl<T: MoveCondition<GeneralUnit>, E: WinCondition<GeneralUnit>> Game<T, E> {
    pub fn new(rules: Rules<GeneralUnit, T, E>) -> Game<T, E> {
        let mut rows = [[None; WIDTH]; HEIGHT];
        for i in 0..ROWS {
            rows[i] = [Some(RED.random_unit()); HEIGHT];
            rows[HEIGHT - i - 1] = [Some(BLUE.random_unit()); HEIGHT];
        }
        let field = Field { rows: rows };
        Game {
            turns: 1,
            current_turn: RED,
            winner: None,
            field: field,
            rules: rules,
        }
    }
    
    pub fn turns(&self) -> u32 { self.turns }
    pub fn current_turn(&self) -> Player { self.current_turn }
    pub fn winner(&self) -> Option<Player> { self.winner }
    pub fn field(&self) -> &Field<GeneralUnit> { &self.field }
    
    pub fn perspective(&self, player: Player) -> PovField {
        PovField::from((&self.field, player))
    }
    
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
        
        if let Some(Some(outcome)) = attack_outcome {
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
        
        Ok(attack_outcome.unwrap())
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
    
    fn unit(&self, rps: RPS) -> GeneralUnit {
        GeneralUnit::new(rps, *self)
    }
    
    fn random_unit(&self) -> GeneralUnit {
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
pub struct Rules<K, T: MoveCondition<K>, E: WinCondition<K>> where K: Unit {
    pub move_condition: T,
    pub win_condition: E,
    phantom_data: PhantomData<K>,
}

impl<K: Unit, T: MoveCondition<K>, E: WinCondition<K>> Rules<K, T, E> {
    pub fn new(move_condition: T, win_condition: E) -> Rules<K, T, E> {
        Rules {
            move_condition: move_condition,
            win_condition: win_condition,
            phantom_data: PhantomData,
        }
    }
}
