extern crate rand;
extern crate rustc_serialize;

/// Width of game board
#[cfg(not(test))] pub const WIDTH: usize = 8;
#[cfg(test)]      pub const WIDTH: usize = 3;
/// Height of game board
#[cfg(not(test))] pub const HEIGHT: usize = 8;
#[cfg(test)]      pub const HEIGHT: usize = 3;
/// Number of rows
#[cfg(not(test))] pub const ROWS: usize = 2;
#[cfg(test)]      pub const ROWS: usize = 1;

const RED: Player = Player::Red;
const BLUE: Player = Player::Blue;

const ROCK: RPS = RPS::Rock;
const PAPER: RPS = RPS::Paper;
const SCISSORS: RPS = RPS::Scissors;

const WIN: Outcome = Outcome::Win;
const LOSE: Outcome = Outcome::Lose;
const DRAW: Outcome = Outcome::Draw;

use rand::{Rng, Rand};

use std::marker::PhantomData;

use moves::{is_valid, Move};
use win_conditions::{WinCondition};
use field::{Field, PovField};
use unit::{Unit, GeneralUnit};

pub mod moves;
pub mod win_conditions;
pub mod unit;
pub mod field;

#[derive(Clone, Debug)]
pub struct Game<T: WinCondition<GeneralUnit>> {
    turns: u32,
    current_turn: Player,
    winner: Option<Player>,
    field: Field<GeneralUnit>,
    rules: Rules<GeneralUnit, T>,
}

impl<T: WinCondition<GeneralUnit>> Game<T> {
    pub fn new(rules: Rules<GeneralUnit, T>) -> Game<T> {
        let mut rows = [[None; WIDTH]; HEIGHT];
        for i in 0..ROWS {
            rows[i] = Self::random_row(RED);
            rows[HEIGHT - i - 1] = Self::random_row(BLUE);
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
    
    fn random_row(player: Player) -> [Option<GeneralUnit>; WIDTH] {
        let mut res = [None; WIDTH];
        for i in 0..WIDTH {
            res[i] = Some(player.random_unit());
        }
        res
    }
    
    pub fn turns(&self) -> u32 { self.turns }
    pub fn current_turn(&self) -> Player { self.current_turn }
    pub fn winner(&self) -> Option<Player> { self.winner }
    pub fn field(&self) -> &Field<GeneralUnit> { &self.field }
    
    pub fn force_win(&mut self, player: Player) {
        if !self.winner.is_some() { self.winner = Some(player); }
    }
    
    pub fn perspective(&self, player: Player) -> PovField {
        PovField::from((&self.field, player))
    }
    
    pub fn make_move(&mut self, movement: Move) -> Result<Option<Outcome>, MoveError> {
        if self.winner.is_some() { return Err(MoveError::GameAlreadyFinished); }
        
        if !is_valid(movement) {
            return Err(MoveError::DeclinedByMoveCondition);
        }
        
        let (from_x, from_y) = movement.from;
        
        if from_x >= WIDTH || from_y >= HEIGHT { return Err(MoveError::PositionOutOfBounds); }
        
        let attack_outcome;
        let (to_x, to_y);
        
        if let Some(ref unit) = self.field.rows[from_y][from_x].as_ref() {
            if unit.owner != self.current_turn { return Err(MoveError::WrongOwner); }
            let dist = movement.apply(unit.owner);
            to_x = dist.0;
            to_y = dist.1; 
            if to_x >= WIDTH || to_y >= HEIGHT { return Err(MoveError::PositionOutOfBounds); }
            
            if let Some(ref defender) = self.field.rows[to_y][to_x].as_ref() {
                if defender.owner == self.current_turn { return Err(MoveError::SameOwner); }
                
                match unit.attack(defender) {
                    Some(res) => {
                        attack_outcome = Some(res);
                    },
                    None => { return Err(MoveError::UnexpextedError); }
                } 
            } else {
                attack_outcome = None;
            }
            
        } else {
            return Err(MoveError::NoUnitInPosition);
        }
        
        if let Some(outcome) = attack_outcome {
            match outcome {
                WIN => {
                    self.field.rows[to_y][to_x] = self.field.rows[from_y][from_x];
                    self.field.rows[from_y][from_x] = None;
                    self.field.rows[to_y][to_x].as_mut().unwrap().visible = true;
                },
                LOSE => {
                    self.field.rows[from_y][from_x] = None;
                    self.field.rows[to_y][to_x].as_mut().unwrap().visible = true;
                },
                DRAW => {
                    let v1 = self.field.rows[from_y][from_x].as_mut().unwrap().visible;
                    let v2 = self.field.rows[to_y][to_x].as_mut().unwrap().visible;
                    
                    if v1 && v2{
                        self.field.rows[from_y][from_x].as_mut().unwrap().rps = rand::random();
                        self.field.rows[to_y][to_x].as_mut().unwrap().rps = rand::random();
                    } else {
                        self.field.rows[from_y][from_x].as_mut().unwrap().visible = true;
                        self.field.rows[to_y][to_x].as_mut().unwrap().visible = true;
                    }
                }
            }
        } else {
            self.field.rows[to_y][to_x] = self.field.rows[from_y][from_x];
            self.field.rows[from_y][from_x] = None;
        }
        
        self.winner = self.rules.win_condition.winner(&self.field);
        self.turns += 1;
        self.current_turn = self.current_turn.next();
        
        Ok(attack_outcome)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveError {
    GameAlreadyFinished,
    DeclinedByMoveCondition,
    PositionOutOfBounds,
    WrongOwner,
    NoUnitInPosition,
    SameOwner,
    UnexpextedError,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
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
        self.unit(rand::random())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
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
}

impl Rand for RPS {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.next_u32() % 3 {
            0 => ROCK,
            1 => PAPER,
            2 => SCISSORS,
            _ => { unreachable!(); }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Clone, Debug)]
pub struct Rules<K, T: WinCondition<K>> where K: Unit {
    pub win_condition: T,
    phantom_data: PhantomData<K>,
}

impl<K: Unit, T: WinCondition<K>> Rules<K, T> {
    pub fn new(win_condition: T) -> Rules<K, T> {
        Rules {
            win_condition: win_condition,
            phantom_data: PhantomData,
        }
    }
}

#[test]
fn basic_test() {
    use moves::{Direction};
    use win_conditions::EliminateCondition;
    
    let rules = Rules::new(EliminateCondition);
    let mut game = Game::new(rules);
    let move1 = Move::new(0, 0, Direction::Forward);
    
    assert_eq!(game.make_move(move1), Ok(None));
    
    assert_eq!(game.make_move(move1), Err(MoveError::NoUnitInPosition));
    
    let move2 = Move::new(0, 2, Direction::Forward);
    assert!(game.make_move(move2).unwrap().is_some());
}
