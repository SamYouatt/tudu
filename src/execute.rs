use crate::date::TuduDate;
use crate::model::{
    AddCommand, Command, EditCommand, SetCommand, Task, TaskList, TaskState, ViewCommand,
};
use crate::TuduError;

pub fn execute_command(command: Command) -> Result<(), TuduError> {
    match command {
        Command::Add(config) => execute_add(config),
        Command::Remove(_) => todo!(),
        Command::Set(config) => execute_set(config),
        Command::Edit(config) => execute_edit(config),
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

fn execute_set(config: SetCommand) -> Result<(), TuduError> {
    let date = match config.date {
        Some(date) => date,
        None => TuduDate::today(),
    };

    let mut task_list = TaskList::for_date(&date)?;

    task_list.set_task_state(config.index, config.state)?;

    Ok(())
}

fn execute_edit(config: EditCommand) -> Result<(), TuduError> {
    // Todo: edit command needs to accept a specific date
    let date = TuduDate::today();

    let mut task_list = TaskList::for_date(&date)?;

    task_list.edit_task(config.index, config.task)?;

    Ok(())
}

fn execute_view(config: ViewCommand) -> Result<(), TuduError> {
    let task_list = TaskList::for_date(&config.date)?;

    let formatted_tasks = task_list.get_formatted_tasks();

    println!("{formatted_tasks}");

    Ok(())
}
