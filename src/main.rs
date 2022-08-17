use clap::Parser;
use std::env;
use std::path::{Path, PathBuf};
use std::process;

mod cli_hndlr;
mod defaults;
mod messages;

use messages::{error, prompt};

struct Files {
    tasks_path: PathBuf,
    temp_path: PathBuf,
}

impl Files {
    fn new(tasks_path: PathBuf, temp_path: PathBuf) -> Files {
        Files {
            tasks_path,
            temp_path,
        }
    }
}

fn main() {
    let filepath: PathBuf;
    let temp_path: PathBuf;
    // Getting filepath from environment variable
    if let Some(files) = get_filepath() {
        filepath = files.tasks_path;
        temp_path = files.temp_path;
    } else {
        eprintln!("{}", error::ENV_VAR_ERR);
        process::exit(1);
    }

    let cli = cli_hndlr::Cli::parse();

    //Adding a task
    if let Some(tasks) = cli.add {
        cli_hndlr::add_task_handler(tasks, &filepath);
        println!("{}", prompt::TASK_ADDED);
        process::exit(0);
    }

    // Marking specific tasks as done
    if let Some(pattern) = cli.mark {
        cli_hndlr::mark_task_handler(pattern, &filepath, &temp_path);
    }

    // Removing specific tasks
    if let Some(pattern) = cli.remove {
        cli_hndlr::remove_task_handler(pattern, &filepath, &temp_path);
    }

    // Removing all marked Tasks
    if cli.remove_marked {
        cli_hndlr::remove_marked_handler(&filepath, &temp_path);
        println!("{}", prompt::DEL_MARKED);
        process::exit(0);
    }

    // Deleting all saved tasks
    if cli.delete {
        cli_hndlr::delete_all_handler(&filepath);
        process::exit(0);
    }

    // Listing all saved tasks
    cli_hndlr::list_task_handler(&filepath);
}

// Tries to gets filepaths from environment variable and converts them to Path
fn get_filepath() -> Option<Files> {
    match env::var(defaults::FILEPATH_ENV_VAR) {
        Ok(f) => {
            let tasks_path = Path::new(&f);
            let mut temp_path = tasks_path.parent()?.to_owned();
            temp_path.push(defaults::DEFAULT_TEMP_FILE);
            return Some(Files::new(tasks_path.to_owned(), temp_path));
        }
        Err(_) => {
            let tasks_path = Path::new(defaults::DEFAULT_TASKS_FILE);
            let temp_path = Path::new(defaults::DEFAULT_TEMP_FILE);
            let files = Files::new(tasks_path.to_owned(), temp_path.to_owned());
            Some(files)
        }
    }
}
