use ::{Player, Field, RED, BLUE};

pub trait WinCondition {
    fn winner(&self, field: &Field) -> Option<Player>;
}

pub struct EliminateCondition;

impl WinCondition for EliminateCondition {
    fn winner(&self, field: &Field) -> Option<Player> {
        let mut red_preseted = false;
        let mut blue_presented = false;
        for &row in field.rows.iter() {
            if red_preseted && blue_presented { break; }
            for &cell in row.iter() {
                if red_preseted && blue_presented { break; }
                if let Some(ref unit) = cell {
                    match unit.owner {
                        RED => red_preseted = true,
                        BLUE => blue_presented = true,
                    }
                }
            }
        }
        match (red_preseted, blue_presented) {
            (true, false) => { Some(RED) },
            (false, true) => { Some(BLUE) },
            _ => None,
        }
    }
}
