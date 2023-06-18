use crate::date::TuduDate;
mod date;
mod error;

#[derive(Eq, PartialEq, Debug)]
enum Command {
    Add(AddCommand),
    Remove(RemoveCommand),
    Set(SetCommand),
    Complete(CompleteCommand),
    View(ViewCommand),
    Root,
}

#[derive(Eq, PartialEq, Debug)]
enum TaskState {
    NotStarted,
    Started,
    Complete,
    Forwarded,
    Ignored,
}

#[derive(Eq, PartialEq, Debug)]
struct AddCommand {
    task: String,
    date: Option<TuduDate>,
}

#[derive(Eq, PartialEq, Debug)]
struct RemoveCommand {
    index: usize,
    date: Option<TuduDate>,
}

#[derive(Eq, PartialEq, Debug)]
struct SetCommand {
    index: usize,
    date: Option<TuduDate>,
    state: TaskState,
}

#[derive(Eq, PartialEq, Debug)]
struct CompleteCommand {
    index: usize,
    date: Option<TuduDate>,
}

#[derive(Eq, PartialEq, Debug)]
struct ViewCommand {
    date: TuduDate,
}

fn parse_add_command(args: Vec<&str>) -> Command {
    let task_arg = args[0].to_owned();

    if !task_arg.starts_with("\"") && !task_arg.ends_with("\"") {
        // Error here
        todo!()
    }

    let task = task_arg.replace("\"", "");

    let date = match args.len() {
        2 => match TuduDate::from_date(args[1]) {
            Ok(date) => Some(date),
            Err(_) => todo!(),
        },
        1 => None,
        _ => todo!(),
    };

    let config = AddCommand { task, date };

    return Command::Add(config);
}

fn parse_remove_command(args: Vec<&str>) -> Command {
    let index = match args[0].parse::<usize>() {
        Ok(index) => index,
        Err(_) => todo!(),
    };

    let date = match args.len() {
        2 => match TuduDate::from_date(args[1]) {
            Ok(date) => Some(date),
            Err(_) => todo!(),
        },
        1 => None,
        // TODO: error here
        _ => todo!(),
    };

    let config = RemoveCommand { index, date };

    return Command::Remove(config);
}

fn parse_command(args: Vec<&str>) -> Command {
    if args.len() == 1 {
        return Command::Root;
    }

    match args[1] {
        "add" => return parse_add_command(args[2..].to_vec()),
        "remove" => return parse_remove_command(args[2..].to_vec()),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_root_command_from_no_args() {
        let args = vec!["tudu"];

        let command = parse_command(args);
        let expected_command = Command::Root;

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

        let command = parse_command(args);

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

        let command = parse_command(args);

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

        let command = parse_command(args);

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

        let command = parse_command(args);
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

        let command = parse_command(args);

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

        let command = parse_command(args);

        assert_eq!(command, expected_command);
    }
}
