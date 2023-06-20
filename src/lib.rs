use error::fail_with_error;
use execute::execute_command;

use crate::date::TuduDate;
use crate::error::TuduError;
use crate::model::{
    AddCommand, Command, EditCommand, RemoveCommand, SetCommand, TaskState, ViewCommand,
};
mod date;
mod error;
mod execute;
mod model;
mod storage;

fn parse_add_command(args: Vec<String>) -> Result<Command, TuduError> {
    let task = args[0].to_owned();

    let date =
        match args.len() {
            2 => Some(TuduDate::from_date(args[1].as_str())?),
            1 => None,
            _ => return Err(TuduError::InvalidArguments(String::from(
                "`add` accepts a task, e.g. \"Example task\", and an optional date, e.g. 10-6-2023",
            ))),
        };

    let config = AddCommand { task, date };

    return Ok(Command::Add(config));
}

fn parse_remove_command(args: Vec<String>) -> Result<Command, TuduError> {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => return Err(TuduError::InvalidIndex),
    };

    let date = match args.len() {
        2 => Some(TuduDate::from_date(args[1].as_str())?),
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

fn parse_set_command(args: Vec<String>) -> Result<Command, TuduError> {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => return Err(TuduError::InvalidIndex),
    };

    let arg_as_str = match args.get(1) {
        Some(value) => value.as_str(),
        None => {
            return Err(TuduError::InvalidArguments(String::from(
                "`set` accepts a task number and a task state, for states see `tudu help`",
            )))
        }
    };

    let state = match arg_as_str {
        "C" => TaskState::Complete,
        "N" => TaskState::NotStarted,
        "S" => TaskState::Started,
        "F" => TaskState::Forwarded,
        "X" => TaskState::Ignored,
        _ => return Err(TuduError::InvalidState),
    };

    let date = match args.len() {
        2 => None,
        3 => Some(TuduDate::from_date(args[2].as_str())?),
        _ => return Err(TuduError::InvalidArguments(
                    String::from("`set` accepts a task number, a task state, and an optional date, e.g. 10-6-2023. For states see `tudu help`")
                )),
    };

    let config = SetCommand { index, state, date };

    return Ok(Command::Set(config));
}

fn parse_complete_command(args: Vec<String>) -> Result<Command, TuduError> {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => return Err(TuduError::InvalidIndex),
    };

    let date = match args.len() {
        1 => None,
        2 => Some(TuduDate::from_date(args[1].as_str())?),
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

fn parse_edit_command(args: Vec<String>) -> Result<Command, TuduError> {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            return Err(TuduError::InvalidArguments(String::from(
                "`edit` accepts a task number and the new task description",
            )))
        }
    };

    let task = args[1].to_owned();

    let config = EditCommand { index, task };

    return Ok(Command::Edit(config));
}

fn parse_view_command(args: Vec<String>) -> Result<Command, TuduError> {
    if args.len() < 1 {
        return Err(TuduError::InvalidArguments(String::from(
            "`view` accepts a date, e.g. 10-6-2023",
        )));
    }

    let date = TuduDate::from_date(args[0].as_str())?;

    let config = ViewCommand { date };

    return Ok(Command::View(config));
}

fn parse_command(args: Vec<String>) -> Result<Command, TuduError> {
    if args.len() == 1 {
        let root_config = ViewCommand {
            date: TuduDate::today(),
        };
        return Ok(Command::View(root_config));
    }

    match args[1].as_str() {
        "add" => parse_add_command(args[2..].to_vec()),
        "remove" => parse_remove_command(args[2..].to_vec()),
        "set" => parse_set_command(args[2..].to_vec()),
        "complete" => parse_complete_command(args[2..].to_vec()),
        "view" => parse_view_command(args[2..].to_vec()),
        "edit" => parse_edit_command(args[2..].to_vec()),
        _ => Err(TuduError::InvalidCommand),
    }
}

pub fn run(args: Vec<String>) {
    let command = match parse_command(args) {
        Ok(command) => command,
        Err(err) => return fail_with_error(err),
    };

    if let Err(err) = execute_command(command) {
        return fail_with_error(err);
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Datelike, Local};

    use super::*;

    #[test]
    fn create_root_command_from_no_args() {
        let args = vec![String::from("tudu")];

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
        let args = vec![
            String::from("tudu"),
            String::from("add"),
            String::from("Example task"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("add"),
            String::from("Example task"),
            String::from("10-6-2023"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("remove"),
            String::from("2"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("remove"),
            String::from("2"),
            String::from("10-6-2023"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("set"),
            String::from("3"),
            String::from("S"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("set"),
            String::from("3"),
            String::from("X"),
            String::from("10-6-2023"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("complete"),
            String::from("2"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("complete"),
            String::from("3"),
            String::from("10-6-2023"),
        ];

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
        let args = vec![
            String::from("tudu"),
            String::from("view"),
            String::from("10-6-2023"),
        ];

        let expected_config = ViewCommand {
            date: TuduDate::new(10, 6, 2023),
        };
        let expected_command = Command::View(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }

    #[test]
    fn create_edit_command() {
        let args = vec![
            String::from("tudu"),
            String::from("edit"),
            String::from("2"),
            String::from("Updated task"),
        ];

        let expected_config = EditCommand {
            index: 2,
            task: String::from("Updated task"),
        };
        let expected_command = Command::Edit(expected_config);

        let command = parse_command(args).unwrap();

        assert_eq!(command, expected_command);
    }
}
