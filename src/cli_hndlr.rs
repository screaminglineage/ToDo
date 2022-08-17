// Handles the CLI parser, calls the required functions and also handles any errors

use clap::{ArgGroup, Parser};
use std::io;
use std::path::Path;
use std::process;

use crate::messages::{error, prompt};
use todo;

#[derive(Parser)]
#[clap(author, version, long_about = None)]
#[clap(about = "Add tasks to a TODO list and then mark them done or remove when required")]
#[clap(group(
    ArgGroup::new("group")
        .args(&["add", "mark", "remove", "remove-marked", "delete"])
    ))]
pub struct Cli {
    /// Add new tasks separated by commas (without any spaces in between)
    #[clap(
        long,
        short,
        value_parser,
        value_name = "NEW-TASK(S)",
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub add: Option<Vec<String>>,

    /// Mark a task as complete.
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to mark multiple tasks at once
    #[clap(long = "mark-done", short = 'x', value_name = "TASK(S)")]
    pub mark: Option<String>,

    /// Remove a specific task.
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to remove multiple tasks at once
    #[clap(long, short, value_name = "TASK(S)")]
    pub remove: Option<String>,

    /// Remove all tasks which have been marked as complete
    #[clap(long, short = 'R', action, value_parser)]
    pub remove_marked: bool,

    /// Delete all tasks
    #[clap(long, short, action, value_parser)]
    pub delete: bool,
}

// Handling Errors

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

// Calls functions and handles errors

// Adds task(s) and handles errors
pub fn add_task_handler(tasks: Vec<String>, filepath: &Path) {
    for task in tasks {
        if let Err(e) = todo::add_task(task, filepath) {
            handle_io_error(e, error::ADD_TASK_ERR);
        };
    }
}

// Marks specific task(s) and handles errors
pub fn mark_task_handler(pattern: String, filepath: &Path, temp_path: &Path) {
    let nums = todo::parse_pattern(pattern);
    if let Err(e) = todo::mark_as_done(nums, filepath, temp_path) {
        handle_not_found_error(e, error::NO_TASKS, error::MARK_TASK_ERR);
    }
}

// Removes specific task(s) and handles errors
pub fn remove_task_handler(pattern: String, filepath: &Path, temp_path: &Path) {
    let nums = todo::parse_pattern(pattern);
    if let Err(e) = todo::remove_task(nums, filepath, temp_path) {
        handle_not_found_error(e, error::NO_TASKS, error::REM_TASK_ERR);
    }
}

// Removes all marked tasks and handles errors
pub fn remove_marked_handler(filepath: &Path, temp_path: &Path) {
    if let Err(e) = todo::remove_marked(filepath, temp_path) {
        handle_not_found_error(e, error::NO_TASKS, error::REM_MARK_TASK_ERR);
    }
}

// Deletes all tasks and handles errors
pub fn delete_all_handler(filepath: &Path) {
    let choice = todo::take_input(prompt::DELETE_ALL);
    match choice.to_lowercase().trim() {
        "y" => {
            if let Err(e) = todo::remove_all(filepath) {
                handle_not_found_error(e, error::NO_TASKS, error::DEL_TASK_ERR);
            }
            println!("{}", prompt::DEL_ALL);
        }
        _ => println!("{}", prompt::DEL_CANCEL),
    }
}

// Lists all tasks and handles errors
pub fn list_task_handler(filepath: &Path) {
    if let Err(e) = todo::display_tasks(filepath) {
        handle_not_found_error(e, error::NO_TASKS_DISPL, error::LIST_TASK_ERR)
    }
}
