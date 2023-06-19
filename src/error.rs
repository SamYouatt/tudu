#[derive(Debug, PartialEq, Eq)]
pub enum TuduError {
    InvalidTask,
    InvalidDate,
    InvalidArguments(String),
    InvalidIndex,
    InvalidState,
    InvalidCommand,
    NoTaskFile,
    FailedToReadFile,
    FailedToWriteFile,
    BadTaskFormat,
    InvalidTaskDirectory,
}
