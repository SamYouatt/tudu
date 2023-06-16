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
