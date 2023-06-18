#[derive(Debug)]
pub enum TuduError {
    InvalidTask,
    InvalidDate,
    InvalidArguments(String),
    UnknownCommand,
}
