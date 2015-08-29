use ::{WIDTH, HEIGHT};
use unit::{Unit};

#[derive(Copy, Clone, Debug)]
pub struct Field<T: Unit + Copy + Clone> {
    pub rows: [[Option<T>; WIDTH]; HEIGHT],
}

impl<T: Unit + Copy + Clone> Field<T> {
    pub fn new() -> Field<T> {
        let rows = [[None; WIDTH]; HEIGHT];
        Field {
            rows: rows,
        }
    }
}
