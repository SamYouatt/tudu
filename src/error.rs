#[derive(Debug)]
pub enum TuduError {
    InvalidTask,
    InvalidDate,
    InvalidArguments(String),
    InvalidIndex,
    InvalidState,
    InvalidCommand,
    NoTaskFile,
    FailedToReadFile,
    BadTaskFormat,
}
