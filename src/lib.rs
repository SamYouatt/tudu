enum Command {
    Add(AddCommand),
    Remove(RemoveCommand),
    Set(SetCommand),
    Complete(CompleteCommand),
    View(ViewCommand),
    Root,
}

enum TaskState {
    NotStarted,
    Started,
    Complete,
    Forwarded,
    Ignored,
}
struct AddCommand {
    task: String,
    date: Option<TuduDate>,
}

struct RemoveCommand {
    index: usize,
    date: Option<TuduDate>,
}

struct SetCommand {
    index: usize,
    date: Option<TuduDate>,
    state: TaskState,
}

struct CompleteCommand {
    index: usize,
    date: Option<TuduDate>,
}

struct ViewCommand {
    date: TuduDate,
}

struct TuduDate {
    date: usize,
    month: usize,
    year: usize,
}
