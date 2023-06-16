enum Command {
    Add,
    Remove,
    Set,
    Complete,
    View(TuduDate),
    Root,
}

enum TaskState {
    NotStarted,
    Started,
    Complete,
    Forwarded,
    Ignored,
}
struct TuduDate {
    date: usize,
    month: usize,
    year: usize,
}
