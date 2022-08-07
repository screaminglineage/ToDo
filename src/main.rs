use clap::{ArgGroup, Parser};
use std::env;
use std::io;
use std::process;

use todo;
mod messages;
use messages::{error, prompt};

const FILEPATH_ENV_VAR: &str = "RTODO_FILE_PATH";
const DEFAULT_FILEPATH: &str = "tasks.txt";
const SEPARATOR: char = '`';

#[derive(Parser)]
#[clap(author, version, long_about = None)]
#[clap(about = "Add tasks to a TODO list and then mark them done or remove when required")]
#[clap(group(
    ArgGroup::new("group")
        .args(&["add", "mark", "remove", "remove-marked", "delete"])
    ))]
struct Cli {
    /// Add new tasks separated by commas (without any spaces in between)
    #[clap(
        long,
        short,
        value_parser,
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    add: Option<Vec<String>>,

    /// Mark a task as complete.
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to mark multiple tasks at once
    #[clap(long = "mark-done", short = 'x', value_name = "TASKS")]
    mark: Option<String>,

    /// Remove a specific task.
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to remove multiple tasks at once
    #[clap(long, short, value_name = "TASKS")]
    remove: Option<String>,

    /// Remove all tasks marked as done
    #[clap(long, short = 'R', action, value_parser)]
    remove_marked: bool,

    /// Delete all tasks
    #[clap(long, short, action, value_parser)]
    delete: bool,
}

fn main() {
    // Getting filepath from environment variable
    let filepath: String;
    if let Ok(env_var) = get_filepath() {
        filepath = env_var;
    } else {
        filepath = DEFAULT_FILEPATH.to_string();
    }

    let cli = Cli::parse();

    //Adding a task
    if let Some(tasks) = cli.add {
        for task in tasks {
            if let Err(e) = todo::add_task(task, &filepath, SEPARATOR) {
                handle_io_error(e, error::ADD_TASK_ERR);
            };
        }
        println!("Task(s) Added");
        process::exit(0);
    }

    // Marking specific tasks as done
    if let Some(pattern) = cli.mark {
        let nums = todo::parse_pattern(pattern);
        if let Err(e) = todo::mark_as_done(nums, &filepath, SEPARATOR) {
            handle_not_found_error(e, error::NO_TASKS, error::MARK_TASK_ERR);
        }
    }

    // Removing specific tasks
    if let Some(pattern) = cli.remove {
        let nums = todo::parse_pattern(pattern);
        if let Err(e) = todo::remove_task(nums, &filepath) {
            handle_not_found_error(e, error::NO_TASKS, error::REM_TASK_ERR);
        }
    }

    // Removing Marked Tasks
    if cli.remove_marked {
        if let Err(e) = todo::remove_marked(&filepath, SEPARATOR) {
            handle_not_found_error(e, error::NO_TASKS, error::REM_MARK_TASK_ERR);
        }
        println!("{}", prompt::DEL_MARKED);
        process::exit(0);
    }

    // Deleting all saved tasks
    if cli.delete {
        let choice = todo::take_input(prompt::DELETE_ALL);
        match choice.to_lowercase().trim() {
            "y" => {
                if let Err(e) = todo::remove_all(&filepath) {
                    handle_not_found_error(e, error::NO_TASKS, error::DEL_TASK_ERR);
                }
                println!("{}", prompt::DEL_ALL);
            }
            _ => println!("{}", prompt::DEL_CANCEL),
        }
        process::exit(0);
    }

    // Listing all saved tasks
    list_tasks(&filepath, SEPARATOR);
}


// Handles IO Error
fn handle_io_error(error: io::Error, desc: &str) {
    eprintln!("{desc} - {error}");
    process::exit(1);
}

// Handles FileNotFound Error
fn handle_not_found_error(error: io::Error, desc_1: &str, desc_2: &str) {
    if error.kind() == io::ErrorKind::NotFound {
        eprintln!("{}", desc_1);
        process::exit(1);
    } else {
        handle_io_error(error, desc_2);
    }
}

// Lists Tasks
fn list_tasks(filepath: &String, separator: char) {
    if let Err(e) = todo::display_tasks(&filepath, separator) {
        handle_not_found_error(e, error::NO_TASKS_DISPL, error::LIST_TASK_ERR)
    }
}

// Gets filepath from environment variable
fn get_filepath() -> Result<String, env::VarError> {
    let filepath = env::var(FILEPATH_ENV_VAR)?;
    Ok(filepath)
}
