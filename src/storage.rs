use crate::error::TuduError;
use crate::model::{Task, TaskState};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn parse_task_file(filename: &PathBuf) -> Result<Vec<Task>, TuduError> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(TuduError::NoTaskFile),
    };

    let mut contents = String::new();

    if let Err(_) = file.read_to_string(&mut contents) {
        return Err(TuduError::FailedToReadFile);
    }

    let tasks = contents
        .lines()
        .map(|line| parse_task_line(line))
        .collect::<Result<Vec<Task>, TuduError>>()?;

    Ok(tasks)
}

fn parse_task_line(line: &str) -> Result<Task, TuduError> {
    let sections: Vec<&str> = line.split(',').collect();

    if sections.len() != 2 {
        return Err(TuduError::BadTaskFormat);
    }

    let state = match sections[0] {
        "S" => TaskState::Started,
        "C" => TaskState::Complete,
        "X" => TaskState::Ignored,
        "F" => TaskState::Forwarded,
        "N" => TaskState::NotStarted,
        _ => return Err(TuduError::FailedToReadFile),
    };

    let task = sections[1].to_owned();

    Ok(Task { task, state })
}

pub fn create_filepath(filename: &str) -> Result<PathBuf, TuduError> {
    let tasks_directory = match env::var("TUDU_TASKS") {
        Ok(path) => path,
        Err(env::VarError::NotPresent) => {
            let home = env::var("HOME").expect("Unable to find HOME environment variable");
            let default_dir = format!("{home}/.tudu");
            build_dir_if_needed(&default_dir)?;
            default_dir
        }
        Err(_) => return Err(TuduError::InvalidTaskDirectory),
    };

    let filepath = PathBuf::from(format!("{tasks_directory}/{filename}"));

    Ok(filepath)
}

pub fn write_tasks_to_file(filename: &PathBuf, tasks: &Vec<Task>) -> Result<(), TuduError> {
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)
    {
        Ok(f) => f,
        Err(_) => return Err(TuduError::FailedToWriteFile),
    };

    let lines = tasks.iter().map(|task| {
        let state = match task.state {
            TaskState::NotStarted => "N",
            TaskState::Started => "S",
            TaskState::Complete => "C",
            TaskState::Forwarded => "F",
            TaskState::Ignored => "X",
        };

        return format!("{},{}\n", state, task.task);
    });

    for line in lines {
        if let Err(_) = file.write_all(line.as_bytes()) {
            return Err(TuduError::FailedToWriteFile);
        }
    }

    Ok(())
}

fn build_dir_if_needed(dir_path: &String) -> Result<(), TuduError> {
    if !Path::is_dir(&PathBuf::from(&dir_path)) {
        return match fs::create_dir_all(dir_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(TuduError::FailedToMakeDirectory),
        };
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_task_file_creates_correct_state() {
        let filename = "./src/tests/2023-06-07.txt";

        let expected_tasks = vec![
            Task::new(String::from("This task is started"), TaskState::Started),
            Task::new(String::from("This one is completed"), TaskState::Complete),
            Task::new(String::from("Didn't like this one"), TaskState::Ignored),
            Task::new(String::from("This one's for later"), TaskState::Forwarded),
            Task::new(String::from("Patience is a virtue"), TaskState::NotStarted),
        ];

        let tasks = parse_task_file(&PathBuf::from(filename)).unwrap();

        assert_eq!(tasks, expected_tasks);
    }

    #[test]
    fn parse_task_line_creates_correct_task() {
        let line = "S,This task is started";

        let expected_task = Task::new(String::from("This task is started"), TaskState::Started);

        let task = parse_task_line(line).unwrap();

        assert_eq!(task, expected_task);
    }

    #[test]
    fn write_tasks_to_file_writes_with_correct_format() {
        let filename = "./src/tests/2023-01-01.txt";

        let tasks = vec![
            Task::new(String::from("This task is started"), TaskState::Started),
            Task::new(String::from("This one is completed"), TaskState::Complete),
            Task::new(String::from("Didn't like this one"), TaskState::Ignored),
            Task::new(String::from("This one's for later"), TaskState::Forwarded),
            Task::new(String::from("Patience is a virtue"), TaskState::NotStarted),
        ];

        let expected_contents = "S,This task is started
C,This one is completed
X,Didn't like this one
F,This one's for later
N,Patience is a virtue
";

        write_tasks_to_file(&PathBuf::from(filename), &tasks).unwrap();

        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, expected_contents);

        // cleanup
        std::fs::remove_file(filename).unwrap();
    }
}
