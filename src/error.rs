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

pub fn fail_with_error(error: TuduError) {
    print_user_error(error);

    std::process::exit(1);
}

fn print_user_error(error: TuduError) {
    match error {
        TuduError::InvalidDate => todo!(),
        TuduError::InvalidArguments(_) => todo!(),
        TuduError::InvalidIndex => todo!(),
        TuduError::InvalidState => todo!(),
        TuduError::InvalidCommand => todo!(),
        TuduError::NoTaskFile => todo!(),
        TuduError::FailedToReadFile => todo!(),
        TuduError::FailedToWriteFile => todo!(),
        TuduError::BadTaskFormat => todo!(),
        TuduError::InvalidTaskDirectory => todo!(),
        TuduError::FailedToMakeDirectory => todo!(),
    }
}
