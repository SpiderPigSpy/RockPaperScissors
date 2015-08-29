use ::{WIDTH, HEIGHT};
use unit::{Unit};

#[derive(Copy, Clone, Debug)]
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
}
