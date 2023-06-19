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

    pub fn add_task(&self, new_task: Task) -> Result<(), TuduError> {
        todo!()
    }

    fn empty(date: &TuduDate) -> TaskList {
        TaskList {
            tasks: Vec::new(),
            date,
        }
    }

    fn write_to_file(self) -> Result<(), TuduError> {
        let filename = self.date.to_filename();

        write_tasks_to_file(&filename, &self.tasks)
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

        let task_list = TaskList {
            date: &date,
            tasks: vec![first_task.clone()],
        };

        task_list.add_task(second_task.clone()).unwrap();

        assert_eq!(task_list.tasks, expected_task_list.tasks);
    }
}
