#[derive(Eq, PartialEq, Debug)]
pub struct TuduDate {
    day: usize,
    month: usize,
    year: usize,
}

impl TuduDate {
    pub fn new(day: usize, month: usize, year: usize) -> TuduDate {
        TuduDate { day, month, year }
    }
}
