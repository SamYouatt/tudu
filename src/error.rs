#[derive(Debug)]
pub enum TuduError<'a> {
    InvalidTask,
    InvalidDate,
    InvalidArguments(&'a str),
    UnknownCommand,
}
