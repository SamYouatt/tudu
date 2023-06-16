use std::{env, fs::File, io::Read, path::PathBuf};

#[derive(Debug)]
enum Command {
    Add,
    Remove,
    Set,
    Complete,
    Date(TodoDate),
    Error(String),
}

enum TaskState {
    Complete,
    Forward,
    NotStarted,
    Started,
    Ignored,
}

struct Task {
    state: TaskState,
    task: String,
}

#[derive(Debug)]
struct TodoDate {
    day: u8,
    month: u8,
    year: u8,
}

#[derive(Debug)]
enum TuduError {
    BadFileFormat,
}

fn main() {
    let mut args = std::env::args();

    if args.len() == 1 {
        println!("This will show todays todo list");
        return;
    }

    let command = args
        .nth(1)
        .expect("expected command if not showing todays list");

    let parsed_command = parse_command(&command);

    let mut file = File::open("/Users/samyouatt/Documents/code/tudu/src/2023-06-15.txt").expect("");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("oops");

    let tasks = parse_tasks_from_contents(&contents).expect("woops");

    for (i, task) in tasks.iter().enumerate() {
        println!("{}", format_task_for_display(task, i));
    }

    let storage_path = match env::var("TUDU_TASKS") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let home = env::var("HOME").expect("Unable to find HOME env var");
            PathBuf::from(format!("{}/.tudu", home))
        }
    };

    println!("Path to tasks: {:?}", storage_path);

    println!("args: {:#?}", args);
}

fn parse_command(command: &str) -> Command {
    return match command {
        "add" => Command::Add,
        "remove" => Command::Remove,
        "set" => Command::Set,
        "complete" => Command::Complete,
        _ if is_relative_date(command) => Command::Date(get_date_from_relative(command)),
        _ => Command::Error(String::from("unknown command")),
    };
}

fn is_relative_date(command: &str) -> bool {
    return match command {
        "today" | "tomorrow" | "yesterday" => true,
        _ => false,
    };
}

fn get_date_from_relative(relative_date: &str) -> TodoDate {
    todo!()
}

fn format_task_for_display(task: &Task, index: usize) -> String {
    let icon = match task.state {
        TaskState::Complete => "●",
        TaskState::Forward => "►",
        TaskState::NotStarted => "◯",
        TaskState::Started => "◐",
        TaskState::Ignored => "x",
    };

    let displayed_index = index + 1;
    let description = &task.task;

    format!("{displayed_index}   {icon} - {description}")
}

fn parse_state(state: &str) -> Result<TaskState, TuduError> {
    return match state {
        "C" => Ok(TaskState::Complete),
        "N" => Ok(TaskState::NotStarted),
        "F" => Ok(TaskState::Forward),
        "X" => Ok(TaskState::Ignored),
        "S" => Ok(TaskState::Started),
        _ => Err(TuduError::BadFileFormat),
    };
}

fn read_task(task_string: &str) -> Result<Task, TuduError> {
    let parts = task_string.split(',').collect::<Vec<&str>>();

    let state = match parts.get(0) {
        Some(s) => parse_state(s)?,
        None => return Err(TuduError::BadFileFormat),
    };

    let task = match parts.get(1) {
        Some(t) => String::from(t.to_owned()),
        None => return Err(TuduError::BadFileFormat),
    };

    Ok(Task { state, task })
}

fn parse_tasks_from_contents(contents: &str) -> Result<Vec<Task>, TuduError> {
    contents.lines().map(|line| read_task(line)).collect()
}
