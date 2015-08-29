use ::{Player, RED, BLUE};
use field::Field;
use unit::Unit;

pub trait WinCondition<T: Unit + Copy + Clone> {
    fn winner(&self, field: &Field<T>) -> Option<Player>;
}

pub struct EliminateCondition;

impl<T: Unit + Copy + Clone> WinCondition<T> for EliminateCondition {
    fn winner(&self, field: &Field<T>) -> Option<Player> {
        let mut red_preseted = false;
        let mut blue_presented = false;
        for &row in field.rows.iter() {
            if red_preseted && blue_presented { break; }
            for &cell in row.iter() {
                if red_preseted && blue_presented { break; }
                if let Some(ref unit) = cell {
                    match unit.owner() {
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
