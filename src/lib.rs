use crate::date::TuduDate;
use crate::error::TuduError;
use crate::model::{AddCommand, Command, RemoveCommand, SetCommand, TaskState, ViewCommand};
mod date;
mod error;
mod model;

fn parse_add_command(args: Vec<&str>) -> Result<Command, TuduError> {
    let task_arg = args[0].to_owned();

    if !task_arg.starts_with("\"") && !task_arg.ends_with("\"") {
        return Err(TuduError::InvalidTask);
    }

    let task = task_arg.replace("\"", "");

    let date =
        match args.len() {
            2 => match TuduDate::from_date(args[1]) {
                Ok(date) => Some(date),
                Err(err) => return Err(err),
            },
            1 => None,
            _ => return Err(TuduError::InvalidArguments(String::from(
                "`add` accepts a task, e.g. \"Example task\", and an optional date, e.g. 10-6-2023",
            ))),
        };

    let config = AddCommand { task, date };

    return Ok(Command::Add(config));
}

fn parse_remove_command(args: Vec<&str>) -> Result<Command, TuduError> {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => return Err(TuduError::InvalidIndex),
    };

    let date = match args.len() {
        2 => match TuduDate::from_date(args[1]) {
            Ok(date) => Some(date),
            Err(err) => return Err(err),
        },
        1 => None,
        _ => {
            return Err(TuduError::InvalidArguments(String::from(
                "`remove` accepts a task number and an optional date e.g. 10-6-2023",
            )))
        }
    };

    let config = RemoveCommand { index, date };

    return Ok(Command::Remove(config));
}

fn parse_set_command(args: Vec<&str>) -> Result<Command, TuduError> {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => return Err(TuduError::InvalidIndex),
    };

    let state = match args.get(1) {
        Some(&"C") => TaskState::Complete,
        Some(&"N") => TaskState::NotStarted,
        Some(&"S") => TaskState::Started,
        Some(&"F") => TaskState::Forwarded,
        Some(&"X") => TaskState::Ignored,
        Some(_) => return Err(TuduError::InvalidState),
        None => {
            return Err(TuduError::InvalidArguments(String::from(
                "`set` accepts a task number and a task state, for states see `tudu help`",
            )))
        }
    };

    let date = match args.len() {
        2 => None,
        3 => match TuduDate::from_date(args[2]) {
            Ok(date) => Some(date),
            Err(err) => return Err(err),
        },
        _ => return Err(TuduError::InvalidArguments(
                    String::from("`set` accepts a task number, a task state, and an optional date, e.g. 10-6-2023. For states see `tudu help`")
                )),
    };

    let config = SetCommand { index, state, date };

    return Ok(Command::Set(config));
}

fn parse_complete_command(args: Vec<&str>) -> Result<Command, TuduError> {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => return Err(TuduError::InvalidIndex),
    };

    let date = match args.len() {
        1 => None,
        2 => match TuduDate::from_date(args[1]) {
            Ok(date) => Some(date),
            Err(err) => return Err(err),
        },
        _ => {
            return Err(TuduError::InvalidArguments(String::from(
                "`complete` accepts a task number and an optional date, e.g. 10-6-2023",
            )))
        }
    };

    let config = SetCommand {
        index,
        date,
        state: TaskState::Complete,
    };

    return Ok(Command::Set(config));
}

fn parse_view_command(args: Vec<&str>) -> Result<Command, TuduError> {
    if args.len() < 1 {
        return Err(TuduError::InvalidArguments(String::from(
            "`view` accepts a date, e.g. 10-6-2023",
        )));
    }

    let date = match TuduDate::from_date(args[0]) {
        Ok(date) => date,
        Err(err) => return Err(err),
    };

    let config = ViewCommand { date };

    return Ok(Command::View(config));
}

pub fn parse_command(args: Vec<&str>) -> Result<Command, TuduError> {
    if args.len() == 1 {
        let root_config = ViewCommand {
            date: TuduDate::today(),
        };
        return Ok(Command::View(root_config));
    }

    match args[1] {
        "add" => parse_add_command(args[2..].to_vec()),
        "remove" => parse_remove_command(args[2..].to_vec()),
        "set" => parse_set_command(args[2..].to_vec()),
        "complete" => parse_complete_command(args[2..].to_vec()),
        "view" => parse_view_command(args[2..].to_vec()),
        _ => Err(TuduError::UnknownCommand),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Datelike, Local};

    use super::*;

    #[test]
    fn create_root_command_from_no_args() {
        let args = vec!["tudu"];

        let now: DateTime<Local> = Local::now();
        let day = now.day();
        let month = now.month();
        let year: u32 = now.year().try_into().unwrap();

        let expected_config = ViewCommand {
            date: TuduDate::new(day, month, year),
        };
        let expected_command = Command::View(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_add_command_with_only_task_when_no_date() {
        let args = vec!["tudu", "add", "\"Example task\""];

        let expected_config = AddCommand {
            task: String::from("Example task"),
            date: None,
        };
        let expected_command = Command::Add(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_add_command_with_date_when_given() {
        let args = vec!["tudu", "add", "\"Example task\"", "10-6-2023"];

        let expected_config = AddCommand {
            task: String::from("Example task"),
            date: Some(TuduDate::new(10, 6, 2023)),
        };
        let expected_command = Command::Add(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_remove_command_without_date() {
        let args = vec!["tudu", "remove", "2"];

        let expected_config = RemoveCommand {
            index: 2,
            date: None,
        };
        let expected_command = Command::Remove(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_remove_command_with_date() {
        let args = vec!["tudu", "remove", "2", "10-6-2023"];

        let expected_config = RemoveCommand {
            index: 2,
            date: Some(TuduDate::new(10, 6, 2023)),
        };
        let expected_command = Command::Remove(expected_config);

        let command = parse_command(args).unwrap();
        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_set_command_without_date() {
        let args = vec!["tudu", "set", "3", "S"];

        let expected_config = SetCommand {
            index: 3,
            state: TaskState::Started,
            date: None,
        };
        let expected_command = Command::Set(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_set_command_with_date() {
        let args = vec!["tudu", "set", "3", "X", "10-6-2023"];

        let expected_config = SetCommand {
            index: 3,
            state: TaskState::Ignored,
            date: Some(TuduDate::new(10, 6, 2023)),
        };
        let expected_command = Command::Set(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_complete_command_without_date() {
        let args = vec!["tudu", "complete", "2"];

        let expected_config = SetCommand {
            index: 2,
            state: TaskState::Complete,
            date: None,
        };
        let expected_command = Command::Set(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_complete_command_with_date() {
        let args = vec!["tudu", "complete", "3", "10-6-2023"];

        let expected_config = SetCommand {
            index: 3,
            state: TaskState::Complete,
            date: Some(TuduDate::new(10, 6, 2023)),
        };
        let expected_command = Command::Set(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_view_command_with_date() {
        let args = vec!["tudu", "view", "10-6-2023"];

        let expected_config = ViewCommand {
            date: TuduDate::new(10, 6, 2023),
        };
        let expected_command = Command::View(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }
}
