use chrono::{DateTime, Datelike, Duration, Local};

use crate::error::TuduError;

#[derive(Eq, PartialEq, Debug)]
pub struct TuduDate {
    day: u32,
    month: u32,
    year: u32,
}

impl TuduDate {
    pub fn new(day: u32, month: u32, year: u32) -> TuduDate {
        TuduDate { day, month, year }
    }

    pub fn from_date(date: &str) -> Result<TuduDate, TuduError> {
        match date {
            "today" => return Ok(TuduDate::today()),
            "tomorrow" => return Ok(TuduDate::tomorrow()),
            "yesterday" => return Ok(TuduDate::yesterday()),
            _ => {}
        };

        let sections: Vec<&str> = date.split("-").collect();

        match sections.len() {
            2 => match (sections[0].parse::<u32>(), sections[1].parse::<u32>()) {
                (Ok(day), Ok(month)) => {
                    if let Err(err) = is_valid_date(day, month) {
                        return Err(err);
                    }

                    // TODO: this 2023 should be the actual year
                    return Ok(TuduDate::new(day, month, 2023));
                }
                _ => return Err(TuduError::InvalidDate),
            },
            3 => match (
                sections[0].parse::<u32>(),
                sections[1].parse::<u32>(),
                sections[2].parse::<u32>(),
            ) {
                (Ok(day), Ok(month), Ok(year)) => {
                    if let Err(err) = is_valid_date(day, month) {
                        return Err(err);
                    }

                    return Ok(TuduDate::new(day, month, year));
                }
                _ => Err(TuduError::InvalidDate),
            },
            _ => return Err(TuduError::InvalidDate),
        }
    }

    pub fn today() -> TuduDate {
        let today = Local::now();

        TuduDate::from_date_time(today)
    }

    pub fn to_filename(&self) -> String {
        format!("{}-{:02}-{:02}.txt", self.year, self.month, self.day)
    }

    fn tomorrow() -> TuduDate {
        let tomorrow = Local::now() + Duration::days(1);

        TuduDate::from_date_time(tomorrow)
    }

    fn yesterday() -> TuduDate {
        let yesterday = Local::now() - Duration::days(1);

        TuduDate::from_date_time(yesterday)
    }

    fn from_date_time(date_time: DateTime<Local>) -> TuduDate {
        let day = date_time.day();
        let month = date_time.month();
        let year = date_time.year().try_into().unwrap();

        TuduDate { day, month, year }
    }
}

fn is_valid_date(day: u32, month: u32) -> Result<(), TuduError> {
    return match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => (day <= 31).then(|| ()).ok_or(TuduError::InvalidDate),
        4 | 6 | 9 | 11 => (day <= 30).then(|| ()).ok_or(TuduError::InvalidDate),
        2 => (day <= 29).then(|| ()).ok_or(TuduError::InvalidDate),
        _ => todo!(),
    };
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

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

    #[test]
    fn from_date_when_given_relative_dates_creates_correct_dates() {
        let today = Local::now();
        let expected_today = TuduDate::today();

        let yesterday = today - Duration::days(1);
        let yesterday_input = format!(
            "{}-{}-{}",
            yesterday.day(),
            yesterday.month(),
            yesterday.year()
        );
        let expected_yesterday = TuduDate::from_date(&yesterday_input);

        let tomorrow = today + Duration::days(1);
        let tomorrow_input = format!(
            "{}-{}-{}",
            tomorrow.day(),
            tomorrow.month(),
            tomorrow.year()
        );
        let expected_tomorrow = TuduDate::from_date(&tomorrow_input);

        let today_date = TuduDate::from_date("today").unwrap();
        let yeterday_date = TuduDate::from_date("yesterday").unwrap();
        let tomorrow_date = TuduDate::from_date("tomorrow").unwrap();

        assert_eq!(today_date, expected_today);
        assert_eq!(today_date, expected_today);
        assert_eq!(today_date, expected_today);
    }

    #[test]
    fn today_gives_todays_date() {
        let now = Local::now();
        let day = now.day();
        let month = now.month();
        let year: u32 = now.year().try_into().unwrap();

        let date = TuduDate::today();

        assert_eq!(day, date.day);
        assert_eq!(month, date.month);
        assert_eq!(year, date.year);
    }

    #[test]
    fn to_filename_generates_correctly_formatted_name() {
        let date = TuduDate::new(7, 6, 2023);

        let expected_filename = String::from("2023-06-07.txt");

        let filename = date.to_filename();

        assert_eq!(filename, expected_filename);
    }
}
