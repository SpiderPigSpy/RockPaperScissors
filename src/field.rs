use ::{Unit, WIDTH, HEIGHT, ROWS, RED, BLUE};

#[derive(Copy, Clone, Debug)]
pub struct Field {
    pub rows: [[Option<Unit>; WIDTH]; HEIGHT],
}

impl Field {
    pub fn new() -> Field {
        let mut rows = [[None; WIDTH]; HEIGHT];
        for i in 0..ROWS {
            rows[i] = [Some(RED.random_unit()); HEIGHT];
            rows[HEIGHT - i - 1] = [Some(BLUE.random_unit()); HEIGHT];
        }
        Field {
            rows: rows,
        }
    }
}
