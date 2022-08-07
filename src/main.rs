use clap::Parser;
use std::env;
use std::process;

mod cli_hndlr;
mod defaults;
mod messages;

use messages::prompt;

fn main() {
    // Getting filepath from environment variable
    let filepath = get_filepath();
    let cli = cli_hndlr::Cli::parse();

    //Adding a task
    if let Some(tasks) = cli.add {
        cli_hndlr::add_task_handler(tasks, &filepath);
        println!("Task(s) Added");
        process::exit(0);
    }

    // Marking specific tasks as done
    if let Some(pattern) = cli.mark {
        cli_hndlr::mark_task_handler(pattern, &filepath);
    }

    // Removing specific tasks
    if let Some(pattern) = cli.remove {
        cli_hndlr::remove_task_handler(pattern, &filepath);
    }

    // Removing all marked Tasks
    if cli.remove_marked {
        cli_hndlr::remove_marked_handler(&filepath);
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

// Gets filepath from environment variable
fn get_filepath() -> String {
    match env::var(defaults::FILEPATH_ENV_VAR) {
        Ok(f) => f,
        Err(_) => defaults::DEFAULT_TASKS_FILE.to_string(),
    }
}
