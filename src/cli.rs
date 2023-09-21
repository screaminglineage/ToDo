// Handles the CLI parser, calls the required functions and also handles any errors

use clap::{ArgGroup, Args, Parser, Subcommand};
use std::io;
use todo::{self, messages, TaskStatus};

#[derive(Parser)]
#[clap(author, version, long_about = None)]
#[clap(about = "Add tasks to a TODO list and then mark them done or remove when required")]
#[clap(group(
    ArgGroup::new("group")
        .args(&["mark", "remove", "remove_marked", "delete_all"])
    ))]
pub struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,

    /// Mark a task as complete
    ///
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to mark multiple tasks at once
    #[clap(long = "mark-done", short = 'x', value_name = "TASK(S)")]
    pub mark: Option<String>,

    /// Unmark a completed task back to incomplete
    ///
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to mark multiple tasks at once
    #[clap(long = "unmark", short = 'u', value_name = "TASK(S)")]
    pub unmark: Option<String>,

    /// Remove a specific task
    ///
    /// A pattern like 1-5,8,10-12 (without spaces)
    /// can also be used to remove multiple tasks at once
    #[clap(long, short, value_name = "TASK(S)")]
    pub remove: Option<String>,

    /// Remove all tasks marked as complete
    #[clap(long, short = 'R', action, value_parser)]
    pub remove_marked: bool,

    /// Delete all tasks
    #[clap(long, short = 'D', action, value_parser)]
    pub delete_all: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new items
    #[clap(visible_alias = "a")]
    Add(Add),
}

#[derive(Args)]
struct Add {
    pub items: Vec<String>,
}

// Deletes all tasks and handles errors
fn confirm_prompt(prompt: &str) -> io::Result<bool> {
    let choice = todo::take_input(prompt)?;
    match choice.to_lowercase().trim() {
        "y" => Ok(true),
        _ => Ok(false),
    }
}

pub fn cli_run(tasks: &mut Vec<todo::Task>) -> Result<(), todo::Error> {
    let cli = Cli::parse();

    // Checking for subcommands
    if let Some(command) = &cli.commands {
        match command {
            Commands::Add(new) => {
                for item in &new.items {
                    todo::add_task(tasks, item);
                }
                println!("{}", messages::TASK_ADDED);
            }
        }
        return Ok(());
    }

    // Removing specific tasks
    if let Some(pattern) = cli.remove {
        let ranges = todo::parse_pattern(&pattern)?;
        todo::remove_tasks(tasks, ranges);
    }

    // Marking specific tasks as done
    if let Some(pattern) = cli.mark {
        let ranges = todo::parse_pattern(&pattern)?;
        todo::change_task_status(tasks, ranges, TaskStatus::Complete);
    }

    // Unmarking specific tasks to set them incomplete
    if let Some(pattern) = cli.unmark {
        let ranges = todo::parse_pattern(&pattern)?;
        todo::change_task_status(tasks, ranges, TaskStatus::Incomplete);
    }

    // Removing all marked Tasks
    if cli.remove_marked {
        todo::remove_completed_tasks(tasks);
    }

    // Deleting all saved tasks
    if cli.delete_all {
        match confirm_prompt(messages::DEL_ALL_ASK) {
            Ok(true) => {
                todo::remove_all(tasks);
                println!("{}", messages::DEL_ALL)
            }
            _ => println!("{}", messages::DEL_CANCEL),
        }
        return Ok(());
    }

    todo::display_tasks(tasks);

    Ok(())
}
