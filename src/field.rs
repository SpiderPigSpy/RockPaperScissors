use std::convert::From;

use ::{Player, WIDTH, HEIGHT};
use unit::{Unit, PovUnit, GeneralUnit};
use move_conditions::{ALL_DIRECTIONS, Move, MoveCondition};

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
    
    pub fn possible_moves(&self, player: Player, move_condition: &MoveCondition) -> Vec<Move> {
        let mut res = Vec::new();
        
        for (j, row) in self.rows.iter().enumerate() {
            for (i, _) in row.iter().enumerate().filter(|&(_, x)| {if let &Some(x) = x {x.owner() == player} else {false} }) {
                for direction in ALL_DIRECTIONS {
                    let movement = Move {from: (i, j), direction: *direction};
                    if move_condition.is_valid(movement) { res.push(movement); }
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
