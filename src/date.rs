use crate::error::TuduError;

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

fn is_valid_date(day: usize, month: usize) -> Result<(), TuduError> {
    return match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => (day <= 31).then(|| ()).ok_or(TuduError::InvalidDate),
        4 | 6 | 9 | 11 => (day <= 30).then(|| ()).ok_or(TuduError::InvalidDate),
        2 => (day <= 29).then(|| ()).ok_or(TuduError::InvalidDate),
        _ => todo!(),
    };
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
