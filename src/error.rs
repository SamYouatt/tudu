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
        TuduError::InvalidDate => println!(
            "The date entered is not a valid date, enter a date in the form of 10-6-2023, or 13-2"
        ),
        TuduError::InvalidArguments(err) => println!("Invalid argument: {err}"),
        TuduError::InvalidIndex => println!("The index entered does not exist for this date"),
        TuduError::InvalidState => {
            println!("The state entered is not valid, see tudu help for more info")
        }
        TuduError::InvalidCommand => {
            println!("The command enetered is not valid, see tudu help for more info")
        }
        TuduError::NoTaskFile => println!(
            "No task file found, if using TUDU_TASKS env variable make sure the folder exists"
        ),
        TuduError::FailedToReadFile => {
            println!("Failed to read file, make sure it has the correct permissions")
        }
        TuduError::FailedToWriteFile => println!(
            "Failed to write tasks to file, make sure the folder has the correct permissions"
        ),
        TuduError::BadTaskFormat => println!("The tasks file was in a bad format"),
        TuduError::InvalidTaskDirectory => {
            println!("The directory specified in TUDU_TASKS is not valid or does not exist")
        }
        TuduError::FailedToMakeDirectory => {
            println!("Failed to create directory, check the permissions for HOME directory")
        }
    }
}
