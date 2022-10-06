// Handles the CLI parser, calls the required functions and also handles any errors

use clap::{ArgGroup, Parser, Subcommand};
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
        .args(&["add", "mark", "remove", "remove_marked", "delete_all"])
    ))]
pub struct Cli {
    /// Add new tasks separated by commas (without any spaces in between)
    /// 
    /// Multiple tasks can be added by being separated by commas,
    /// in the format "Task 1","Task 2","Task 3".
    /// Note that there are no spaces in between each task
    #[clap(
        long,
        short,
        value_parser,
        value_name = "NEW-TASK(S)",
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub add: Option<Vec<String>>,

    /// Mark a task as complete
    ///
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to mark multiple tasks at once
    #[clap(long = "mark-done", short = 'x', value_name = "TASK(S)")]
    pub mark: Option<String>,

    /// Remove a specific task
    ///
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to remove multiple tasks at once
    #[clap(long, short, value_name = "TASK(S)")]
    pub remove: Option<String>,

    /// Remove all tasks which have been marked as complete
    #[clap(long, short = 'R', action, value_parser)]
    pub remove_marked: bool,

    /// Delete all tasks
    #[clap(long, short = 'D', action, value_parser)]
    pub delete_all: bool,

    /// Subcommand
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch ToDo in TUI mode
    Tui {
        #[clap(action)]
        tui: Option<bool>,
    },
}

// Handling Errors

// Handles IO Error
pub fn handle_io_error(error: io::Error, desc: &str) {
    eprintln!("{desc} - {error}");
    process::exit(1);
}

// Handles FileNotFound Error
pub fn handle_not_found_error(error: io::Error, cause_desc: &str, result_desc: &str) {
    if error.kind() == io::ErrorKind::NotFound {
        eprintln!("{}", cause_desc);
        process::exit(1);
    } else {
        handle_io_error(error, result_desc);
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
    if let Err(e) = todo::mark_done(nums, filepath, temp_path) {
        handle_not_found_error(e, error::NO_TASKS, error::MARK_TASK_ERR);
    }
}

// Removes specific task(s) and handles errors
pub fn remove_task_handler(pattern: String, filepath: &Path, temp_path: &Path) {
    let nums = todo::parse_pattern(pattern);
    if let Err(e) = todo::remove_tasks(nums, filepath, temp_path) {
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
