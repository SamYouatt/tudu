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

#[derive(Eq, PartialEq, Debug)]
struct TuduDate {
    date: usize,
    month: usize,
    year: usize,
}

fn parse_command(args: Vec<&str>) -> Command {
    todo!()
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
}
