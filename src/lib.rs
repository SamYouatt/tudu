enum Command {
    Add,
    Remove,
    Set,
    Complete,
    Date(TuduDate),
    Root,
}

struct TuduDate {
    date: usize,
    month: usize,
    year: usize,
}
