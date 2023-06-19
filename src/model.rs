use std::env;
use std::path::PathBuf;

use crate::storage::{parse_task_file, write_tasks_to_file};
use crate::TuduDate;
use crate::TuduError;

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Add(AddCommand),
    Remove(RemoveCommand),
    Set(SetCommand),
    View(ViewCommand),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TaskState {
    NotStarted,
    Started,
    Complete,
    Forwarded,
    Ignored,
}

#[derive(Eq, PartialEq, Debug)]
pub struct AddCommand {
    pub task: String,
    pub date: Option<TuduDate>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct RemoveCommand {
    pub index: usize,
    pub date: Option<TuduDate>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct SetCommand {
    pub index: usize,
    pub date: Option<TuduDate>,
    pub state: TaskState,
}

#[derive(Eq, PartialEq, Debug)]
pub struct CompleteCommand {
    pub index: usize,
    pub date: Option<TuduDate>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ViewCommand {
    pub date: TuduDate,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    pub task: String,
    pub state: TaskState,
}

impl Task {
    pub fn new(task: String, state: TaskState) -> Task {
        Task { task, state }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TaskList<'a> {
    tasks: Vec<Task>,
    date: &'a TuduDate,
}

impl TaskList<'_> {
    pub fn for_date(date: &TuduDate) -> Result<TaskList, TuduError> {
        let filename = date.to_filename();

        match parse_task_file(&filename) {
            Ok(tasks) => Ok(TaskList { tasks, date }),
            Err(TuduError::NoTaskFile) => Ok(TaskList::empty(date)),
            Err(err) => Err(err),
        }
    }

    pub fn add_task(&mut self, new_task: Task) {
        self.tasks.push(new_task);
    }

    pub fn set_task_state(
        &mut self,
        index: usize,
        desired_state: TaskState,
    ) -> Result<(), TuduError> {
        let corrected_index = index - 1;

        match self.tasks.get_mut(corrected_index) {
            Some(task) => Ok(task.state = desired_state),
            None => Err(TuduError::InvalidIndex),
        }
    }

    pub fn remove_task(&mut self, index: usize) {
        let corrected_index = index - 1;

        self.tasks.remove(corrected_index);
    }

    fn empty(date: &TuduDate) -> TaskList {
        TaskList {
            tasks: Vec::new(),
            date,
        }
    }

    fn write_to_file(&self) -> Result<(), TuduError> {
        let filename = self.date.to_filename();

        let tasks_directory = match env::var("TUDU_ERROR") {
            Ok(path) => path,
            Err(env::VarError::NotPresent) => {
                let home = env::var("HOME").expect("Unable to find HOME environment variable");
                format!("{home}/.tudu")
            }
            Err(_) => return Err(TuduError::InvalidTaskDirectory),
        };

        let filepath = PathBuf::from(format!("{tasks_directory}/{filename}"));

        write_tasks_to_file(&filepath, &self.tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_file_when_no_file_creates_empty_task_list() {
        let date = TuduDate::new(2023, 12, 13);

        let expected_task_list = TaskList::empty(&date);

        let task_list = TaskList::for_date(&date).unwrap();

        assert_eq!(task_list, expected_task_list);
    }

    #[test]
    fn add_task_adds_task_to_end_of_list() {
        let date = TuduDate::new(1, 1, 2023);
        let first_task = Task::new(String::from("First task"), TaskState::Complete);
        let second_task = Task::new(String::from("Second task"), TaskState::NotStarted);

        let expected_task_list = TaskList {
            date: &date,
            tasks: vec![first_task.clone(), second_task.clone()],
        };

        let mut task_list = TaskList {
            date: &date,
            tasks: vec![first_task.clone()],
        };

        task_list.add_task(second_task.clone());

        assert_eq!(task_list.tasks, expected_task_list.tasks);
    }

    #[test]
    fn set_task_at_index_edits_that_task() {
        let date = TuduDate::new(1, 1, 2023);
        let first_task = Task::new(String::from("AAA"), TaskState::Complete);
        let second_task = Task::new(String::from("BBB"), TaskState::Complete);

        let expected_task_list = TaskList {
            tasks: vec![first_task.clone(), second_task.clone()],
            date: &date,
        };

        let mut task_list = TaskList {
            date: &date,
            tasks: vec![
                first_task.clone(),
                Task::new(String::from("BBB"), TaskState::NotStarted),
            ],
        };

        task_list.set_task_state(2, TaskState::Complete).unwrap();

        assert_eq!(task_list.tasks, expected_task_list.tasks);
    }

    #[test]
    fn set_task_at_index_if_no_task_at_index_throws_error() {
        let date = TuduDate::new(1, 1, 2023);

        let mut task_list = TaskList {
            date: &date,
            tasks: vec![Task::new(String::from("AAA"), TaskState::NotStarted)],
        };

        let expected_error = TuduError::InvalidIndex;

        let result = task_list.set_task_state(2, TaskState::Complete);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), expected_error);
    }

    #[test]
    fn remove_task_at_index_removes_that_task() {
        let date = TuduDate::new(1, 1, 2023);
        let first_task = Task::new(String::from("AAA"), TaskState::Complete);
        let second_task = Task::new(String::from("BBB"), TaskState::Complete);

        let expected_task_list = TaskList {
            tasks: vec![first_task.clone()],
            date: &date,
        };

        let mut task_list = TaskList {
            date: &date,
            tasks: vec![first_task, second_task],
        };

        task_list.remove_task(2);

        assert_eq!(task_list.tasks, expected_task_list.tasks);
    }
}
