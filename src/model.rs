use crate::TuduDate;

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Add(AddCommand),
    Remove(RemoveCommand),
    Set(SetCommand),
    View(ViewCommand),
}

#[derive(Eq, PartialEq, Debug)]
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
