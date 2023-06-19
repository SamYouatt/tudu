use crate::date::TuduDate;
use crate::error::TuduError;
use crate::model::TaskState;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Eq)]
struct Task {
    task: String,
    state: TaskState,
}

impl Task {
    pub fn new(task: String, state: TaskState) -> Task {
        Task { task, state }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TaskList<'a> {
    tasks: Vec<Task>,
    date: &'a TuduDate,
}

impl TaskList {
    pub fn empty() -> TaskList {
        TaskList { tasks: Vec::new() }
    }

    pub fn for_date(date: &TuduDate) -> Result<TaskList, TuduError> {
        let filename = date.to_filename();

        match parse_task_file(&filename) {
            Ok(task_list) => Ok(task_list),
            Err(TuduError::NoTaskFile) => Ok(TaskList::empty()),
            Err(err) => Err(err),
        }
    }
}

fn parse_task_file(filename: &str) -> Result<TaskList, TuduError> {
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

    Ok(TaskList { tasks })
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

        let tasks = parse_task_file(filename).unwrap();

        assert_eq!(tasks, expected_tasks);
    }

    #[test]
    fn create_from_file_when_no_file_creates_empty_task_list() {
        let date = TuduDate::new(2023, 12, 13);

        let expected_task_list = TaskList::empty(&date);

        let task_list = TaskList::for_date(&date).unwrap();

        assert_eq!(task_list, expected_task_list);
    }

    #[test]
    fn parse_task_line_creates_correct_task() {
        let line = "S,This task is started";

        let expected_task = Task::new(String::from("This task is started"), TaskState::Started);

        let task = parse_task_line(line).unwrap();

        assert_eq!(task, expected_task);
    }
}
