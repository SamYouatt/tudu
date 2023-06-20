use crate::date::TuduDate;
use crate::model::{AddCommand, Command, Task, TaskList, TaskState, ViewCommand};
use crate::TuduError;

pub fn execute_command(command: Command) -> Result<(), TuduError> {
    match command {
        Command::Add(config) => execute_add(config),
        Command::Remove(_) => todo!(),
        Command::Set(_) => todo!(),
        Command::Edit(_) => todo!(),
        Command::View(config) => execute_view(config),
    }
}

fn execute_add(config: AddCommand) -> Result<(), TuduError> {
    let date = match config.date {
        Some(date) => date,
        None => TuduDate::today(),
    };

    let new_task = Task::new(config.task, TaskState::NotStarted);

    let mut task_list = TaskList::for_date(&date)?;

    task_list.add_task(new_task);

    Ok(())
}

fn execute_view(config: ViewCommand) -> Result<(), TuduError> {
    let task_list = TaskList::for_date(&config.date)?;

    let formatted_tasks = task_list.get_formatted_tasks();

    println!("{formatted_tasks}");

    Ok(())
}
