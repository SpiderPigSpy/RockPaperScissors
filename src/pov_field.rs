use std::convert::From;

use ::{Player, WIDTH, HEIGHT};
use unit::{PovUnit, GeneralUnit};
use field::Field;

pub struct PovField {
    pub pov: Player,
    pub rows: [[Option<PovUnit>; WIDTH]; HEIGHT],
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
            rows: rows,
        }
    }
}
