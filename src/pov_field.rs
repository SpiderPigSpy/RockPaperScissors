use std::convert::From;

use ::{Player, RPS, Unit, Field, WIDTH, HEIGHT};

pub struct PovField {
    pub pov: Player,
    pub rows: [[Option<PovUnit>; WIDTH]; HEIGHT],
}

impl<'a> From<(&'a Field, Player)> for PovField {
    fn from( data: (&Field, Player) ) -> PovField {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PovUnit {
    Ally(AllyUnit),
    Enemy(EnemyUnit),
}

impl From<(Unit, Player)> for PovUnit {
    fn from( data: (Unit, Player) ) -> PovUnit {
        let (ref unit, ref player) = data;
        
        if unit.owner == *player {
            PovUnit::Ally( AllyUnit { rps: unit.rps, visible: unit.visible } )
        } else {
            PovUnit::Enemy( EnemyUnit { rps: if unit.visible { Some(unit.rps) } else { None } } )
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllyUnit {
    rps: RPS,
    visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnemyUnit {
    rps: Option<RPS>,
}
