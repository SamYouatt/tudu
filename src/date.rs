use std::error::Error;

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

    pub fn from_date(date: &str) -> Result<TuduDate, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_date_when_given_full_date_creates_correct_date() {
        let input_date = "10-6-2023";

        let expected_date = TuduDate::new(10, 6, 2023);

        let date = TuduDate::from_date(input_date).unwrap();

        assert_eq!(date, expected_date);
    }

    #[test]
    fn from_date_when_given_yearless_date_creates_correct_date() {
        let input_date = "12-3";

        let expected_date = TuduDate::new(12, 3, 2023);

        let date = TuduDate::from_date(input_date).unwrap();

        assert_eq!(date, expected_date);
    }
}
