use std::fs::File;
use std::io::Read;

use crate::error::TuduError;
use crate::model::TaskState;

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
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn empty() -> TaskList {
        TaskList { tasks: Vec::new() }
    }
}

fn parse_task_line(line: &str) -> Result<Task, TuduError> {
}

fn parse_task_file(filename: &str) -> Result<TaskList, TuduError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_task_file_creates_correct_state() {
        let test_file = "./src/tests/2023-06-07.txt";

        let expected_tasks = vec![
            Task::new(String::from("This task is started"), TaskState::Started),
            Task::new(String::from("This one is completed"), TaskState::Complete),
            Task::new(String::from("Didn't like this one"), TaskState::Ignored),
            Task::new(String::from("This one's for later"), TaskState::Forwarded),
            Task::new(String::from("Patience is a virtue"), TaskState::NotStarted),
        ];
        let expected_task_list = TaskList {
            tasks: expected_tasks,
        };

        let task_list = parse_task_file(test_file).unwrap();

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
