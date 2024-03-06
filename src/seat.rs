pub struct Seat<'a> {
    grade: &'a str,
	row: u32,
	col: u32,
}

impl <'a> Seat<'a> {
    pub fn new(grade: &'a str, row: u32, col: u32) -> Seat<'a> {
        Seat{ grade, row, col }
    }
}