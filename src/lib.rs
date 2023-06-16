enum Command {
    Add,
    Remove,
    Set,
    Complete,
    View(TuduDate),
    Root,
}

struct TuduDate {
    date: usize,
    month: usize,
    year: usize,
}
