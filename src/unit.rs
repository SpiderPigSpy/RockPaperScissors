use ::{Player, RPS, Outcome};

pub trait Unit: Copy + Clone {
    fn owner(&self) -> Player;
    fn rps(&self) -> Option<RPS>;
    fn attack(&self, opponent: &Self) -> Option<Outcome> {
        match (self.rps(), opponent.rps()) {
            (Some(attacker), Some(defender)) => Some(attacker.attack(defender)),
            _ => None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneralUnit {
    pub rps: RPS,
    pub owner: Player,
    pub visible: bool,
}

impl GeneralUnit {
    pub fn new(rps: RPS, owner: Player,) -> GeneralUnit {
        GeneralUnit {
            rps: rps,
            owner: owner,
            visible: false,
        }
    }
    
}

impl Unit for GeneralUnit {
    fn owner(&self) -> Player { self.owner }
    fn rps(&self) -> Option<RPS> { Some(self.rps) }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PovUnit {
    pub owner: Player,
    pub part: Part,
}

impl Unit for PovUnit {
    fn owner(&self) -> Player {
        self.owner
    }
    
    fn rps(&self) -> Option<RPS> {
        match self.part {
            Part::Ally(unit) => { Some(unit.rps) },
            Part::Enemy(unit) => { unit.rps },
        }
    }
}

impl From<(GeneralUnit , Player)> for PovUnit {
    fn from( data: (GeneralUnit, Player) ) -> PovUnit {
        let (ref unit, ref player) = data;
        
        if unit.owner() == *player {
            PovUnit {
                owner: unit.owner(),
                part: Part::Ally( AllyUnit { rps: unit.rps, visible: unit.visible } )
            }
        } else {
            PovUnit {
                owner: unit.owner(),
                part: Part::Enemy( EnemyUnit { rps: if unit.visible { Some(unit.rps) } else { None } } )
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Ally(AllyUnit),
    Enemy(EnemyUnit),
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
