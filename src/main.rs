use clap::{ArgGroup, Parser};
use std::env;
use std::io;
use std::process;

use todo;

const FILEPATH_ENV_VAR: &str = "RTODO_FILE_PATH";
const DEFAULT_FILEPATH: &str = "tasks.txt";
const SEPARATOR: char = '`';

#[derive(Parser)]
#[clap(author, version, long_about = None)]
#[clap(about = "Add tasks to a TODO list and then mark them done or remove when required")]
#[clap(group(
    ArgGroup::new("group")
        .args(&["add", "list", "mark", "remove", "delete"])
    ))]
struct Cli {
    /// Add a new task
    #[clap(long, short, value_parser)]
    add: Option<String>,

    /// Lists all tasks when no options are given
    #[clap(value_name = "...")]
    list: Option<String>,

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

    /// Delete all tasks
    #[clap(long, short, action, value_parser)]
    delete: bool,
}

fn main() {
    // Getting filepath from environment variable
    let filepath: String;
    if let Ok(v) = get_filepath() {
        filepath = v;
    } else {
        filepath = DEFAULT_FILEPATH.to_string();
    }

    let cli = Cli::parse();

    // Adding a task
    if let Some(task) = cli.add {
        println!("Task Added");
        if let Err(e) = todo::add_task(task, &filepath, SEPARATOR) {
            handle_io_error(e, "Error in Adding Task");
        };
        process::exit(0);
    }

    // Marking specific tasks as done
    if let Some(pattern) = cli.mark {
        let nums = todo::parse_pattern(pattern);
        if let Err(e) = todo::mark_as_done(nums, &filepath, SEPARATOR) {
            handle_not_found_error(e, "No Saved Tasks Found!", "Error in Marking Tasks");
        }
    }

    // Removing specific tasks
    if let Some(pattern) = cli.remove {
        let nums = todo::parse_pattern(pattern);
        if let Err(e) = todo::remove_task(nums, &filepath) {
            handle_not_found_error(e, "No Saved Tasks Found!", "Error in Marking Tasks");
        }
    }

    // Deleting all saved tasks
    if cli.delete {
        let choice = todo::take_input("Do you want to delete all saved tasks (y/N): ");
        match choice.to_lowercase().trim() {
            "y" => {
                if let Err(e) = todo::remove_all(&filepath) {
                    handle_not_found_error(e, "No Saved Tasks Found!", "Error in Deleting Tasks");
                }
                println!("All Saved Tasks Deleted");
            }
            _ => println!("Tasks Left Unchanged"),
        }
        process::exit(0);
    }

    // Listing all saved tasks
    list_tasks(&filepath, SEPARATOR);
}

fn handle_io_error(error: io::Error, desc: &str) {
    eprintln!("{desc} - {error}");
    process::exit(1);
}

fn handle_not_found_error(error: io::Error, desc_1: &str, desc_2: &str) {
    if error.kind() == io::ErrorKind::NotFound {
        eprintln!("{}", desc_1);
        process::exit(1);
    } else {
        handle_io_error(error, desc_2);
    }
}

fn list_tasks(filepath: &String, separator: char) {
    if let Err(e) = todo::display_tasks(&filepath, separator) {
        handle_not_found_error(e, "No Tasks to Display!", "Error in Displaying Tasks")
    }
}

fn get_filepath() -> Result<String, env::VarError> {
    let filepath = env::var(FILEPATH_ENV_VAR)?;
    Ok(filepath)
}
