use crate::date::TuduDate;

fn filename_from_date(date: &TuduDate) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::TuduDate;

    #[test]
    fn filename_from_date_gives_correct_name() {
        let date = TuduDate::new(10, 6, 2023);

        let expected_filename = String::from("2023-06-10.txt");

        let filename = filename_from_date(&date);

        assert_eq!(filename, expected_filename);
    }
}
