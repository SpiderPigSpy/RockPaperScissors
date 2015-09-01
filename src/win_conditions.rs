use ::{Player, RED, BLUE, HEIGHT};
use field::Field;
use unit::Unit;

pub trait WinCondition<T: Unit> {
    fn winner(&self, field: &Field<T>) -> Option<Player>;
}

#[derive(Clone, Copy, Debug)]
pub struct EliminateCondition;

impl<T: Unit> WinCondition<T> for EliminateCondition {
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

#[derive(Clone, Copy, Debug)]
pub struct GetToLastRowCondition;

impl<T: Unit> WinCondition<T> for GetToLastRowCondition {
    fn winner(&self, field: &Field<T>) -> Option<Player> {
        
        fn unit_check<T: Unit>(cell: &Option<T>, player: Player) -> bool {
            if let Some(ref u) = cell.as_ref() {
                u.owner() == player
            } else {
                false
            }
        };    
    
        let blue_at_first = field.rows[0].iter().any(|&x| unit_check(&x, BLUE) );
        let red_at_last = field.rows[HEIGHT - 1].iter().any(|&x| unit_check(&x, RED) );
        
        match (red_at_last, blue_at_first) {
            (true, false) => Some(RED),
            (false, true) => Some(BLUE),
            _ => None
        }
    }
}
