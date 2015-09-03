use std::convert::From;

use ::{Player, WIDTH, HEIGHT};
use unit::{Unit, PovUnit, GeneralUnit};
use moves::{is_valid, ALL_DIRECTIONS, Move};

#[derive(Copy, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Field<T: Unit> {
    pub rows: [[Option<T>; WIDTH]; HEIGHT],
}

impl<T: Unit> Field<T> {
    pub fn new() -> Field<T> {
        let rows = [[None; WIDTH]; HEIGHT];
        Field {
            rows: rows,
        }
    }
    
    pub fn possible_moves(&self, player: Player) -> Vec<Move> {
        let mut res = Vec::new();
        
        for (j, row) in self.rows.iter().enumerate() {
            for (i, _) in row.iter().enumerate().filter(|&(_, x)| {if let &Some(x) = x {x.owner() == player} else {false} }) {
                for direction in ALL_DIRECTIONS {
                    let movement = Move {from: (i, j), direction: *direction};
                    if is_valid(movement) { res.push(movement); }
                }
            } 
        }
        
        res
    }
}

#[derive(Copy, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct PovField {
    pub pov: Player,
    pub field: Field<PovUnit>,
}

impl PovField {
    pub fn possible_moves(&self) -> Vec<Move> {
        self.field.possible_moves(self.pov)
    }
}

impl<'a> From<(&'a Field<GeneralUnit>, Player)> for PovField {
    fn from( data: (&Field<GeneralUnit>, Player) ) -> PovField {
        let (ref field, player) = data;
        let mut rows = [[None; WIDTH]; HEIGHT];
        
        for (i, ref row) in field.rows.iter().enumerate() {
            for (j, unit) in row.iter().enumerate() {
                if let Some(unit) = *unit {
                    rows[i][j] = Some( PovUnit::from( (unit, player) ) );
                }
            }
        }
        
        PovField {
            pov: player,
            field: Field{ rows: rows },
        }
    }
}

#[test]
fn possible_moves_test() {
    use moves::{Direction};
    use ::RPS;
    
    let mut field = Field::new();
    field.rows[0][0] = Some(GeneralUnit { rps: RPS::Rock, owner: Player::Blue, visible: false,});
    field.rows[0][1] = Some(GeneralUnit { rps: RPS::Rock, owner: Player::Red, visible: false,});
    field.rows[1][0] = Some(GeneralUnit { rps: RPS::Rock, owner: Player::Red, visible: false,});
    let moves = vec![Move {from: (1, 0), direction: Direction::Forward},
                     Move {from: (1, 0), direction: Direction::ForwardLeft},
                     Move {from: (1, 0), direction: Direction::ForwardRight},
                     Move {from: (0, 1), direction: Direction::Forward},
                     Move {from: (0, 1), direction: Direction::ForwardLeft},
                     Move {from: (0, 1), direction: Direction::ForwardRight}];
    assert_eq!(field.possible_moves(Player::Red), moves);
}
