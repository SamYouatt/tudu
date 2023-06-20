use crate::date::TuduDate;
use crate::model::{
    AddCommand, Command, EditCommand, RemoveCommand, SetCommand, Task, TaskList, TaskState,
    ViewCommand,
};
use crate::TuduError;

pub fn execute_command(command: Command) -> Result<(), TuduError> {
    match command {
        Command::Add(config) => execute_add(config),
        Command::Remove(config) => execute_remove(config),
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

    task_list.write_to_file()?;

    print_tasks(&task_list);

    Ok(())
}

fn execute_remove(config: RemoveCommand) -> Result<(), TuduError> {
    let date = match config.date {
        Some(date) => date,
        None => TuduDate::today(),
    };

    let mut task_list = TaskList::for_date(&date)?;

    task_list.remove_task(config.index)?;

    task_list.write_to_file()?;

    print_tasks(&task_list);

    Ok(())
}

fn execute_set(config: SetCommand) -> Result<(), TuduError> {
    let date = match config.date {
        Some(date) => date,
        None => TuduDate::today(),
    };

    let mut task_list = TaskList::for_date(&date)?;

    task_list.set_task_state(config.index, config.state)?;

    task_list.write_to_file()?;

    print_tasks(&task_list);

    Ok(())
}

fn execute_edit(config: EditCommand) -> Result<(), TuduError> {
    // Todo: edit command needs to accept a specific date
    let date = TuduDate::today();

    let mut task_list = TaskList::for_date(&date)?;

    task_list.edit_task(config.index, config.task)?;

    task_list.write_to_file()?;

    print_tasks(&task_list);

    Ok(())
}

fn execute_view(config: ViewCommand) -> Result<(), TuduError> {
    let task_list = TaskList::for_date(&config.date)?;

    print_tasks(&task_list);

    Ok(())
}

fn execute_help() {
    println!("Commands:");
    println!("`tudu` - see the tasks for today");
    println!("`tudu view [date]` - see tasks on given date");
    println!("`tudu add [task] *[date]` - add specified task on optional date");
    println!("`tudu set [index] [state] *[date]` - set specified task to provided state on optional date");
    println!("`tudu complete [index] *[date]` - mark specified task as complete on optional date");
    println!("`tudu edit [index] [task] *[date]` - edit specified task to new task description on optional date");
    println!("`tudu remove [index] *[date]` - remove specified task on optional date");
    println!("");
    println!("Dates:");
    println!("Dates can be written in the form 10-6-2023, 10-6 which uses the current year, or with relative date commands `yesterday/today/tomorrow`.");
    println!("If a date is optional in a command and was not specified the command will use the current date");
    println!("");
    println!("States:");
    println!("◯ - [N]ot started");
    println!("◐ - [S]tarted");
    println!("● - [C]ompleted");
    println!("► - Carry [F]orward");
    println!("x - [X] Not doing");
}

fn print_tasks(task_list: &TaskList) {
    let formatted_tasks = task_list.get_formatted_tasks();

    println!("{formatted_tasks}");
}
