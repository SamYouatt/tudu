#[derive(Debug, PartialEq, Eq)]
pub enum TuduError {
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
    FailedToMakeDirectory,
}
