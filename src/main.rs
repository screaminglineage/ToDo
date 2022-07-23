use clap::{ArgGroup, Parser};
use std::process;
use std::io;

use todo;

const FILEPATH: &str = "tasks.txt";
const SEPARATOR: char = '`';

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("group")
        .required(true)
        .args(&["task", "list", "mark", "remove"])
    ))]
struct Cli {
    /// Add a new task
    #[clap(value_parser)]
    task: Option<String>,

    /// List all tasks
    #[clap(long, short, action, value_parser)]
    list: bool,

    /// Mark a task as complete
    #[clap(long="mark-done", short='x', value_name="TASKS")]
    mark: Option<String>,

    /// Remove all tasks
    #[clap(long, short, action, value_parser)]
    remove: bool,
}

fn main() {
    let cli = Cli::parse();

    // Adding a task
    if let Some(task) = cli.task {
        println!("Adding - {task}"); // testing code
        if let Err(e) = todo::add_task(task, FILEPATH, SEPARATOR) {
            handle_io_error(e, "Error in Adding Task");
        };
    }

    // Listing all saved tasks
    if cli.list {
        list_tasks(FILEPATH, SEPARATOR);
    }

    // Marking specific task as done
    if let Some(pattern) = cli.mark {
        println!("Marking task {:?} as done", pattern); //testing code
        let nums = todo::parse_pattern(pattern);
        if let Err(e) = todo::mark_as_done(nums, FILEPATH, SEPARATOR) {
            handle_io_error(e, "Error in Marking Task Done");
        }
        list_tasks(FILEPATH, SEPARATOR);
    }

    // Removing all saved tasks
    if cli.remove {
        let choice = todo::take_input(
            "Do you want to remove all saved tasks (y/n): "
        );
        match choice.to_lowercase().trim() {
            "y" => {
                println!("Removing all saved tasks"); 
                if let Err(e) = todo::remove_all(FILEPATH) {
                    handle_io_error(e, "Error in Deleting Tasks");
                }
            },
            _ => {
                println!("No tasks deleted"); 
                process::exit(0);
            }
        }
    }
}


fn handle_io_error(error: io::Error, desc: &str) {
    eprintln!("{desc} - {error}");
    process::exit(1);
}


fn list_tasks(filepath: &str, separator: char) {
    println!("Listing all tasks"); // testing code
    if let Err(e) = todo::display_tasks(filepath, separator) {
        if e.kind() == io::ErrorKind::NotFound {
            eprintln!("No tasks to display!");
            process::exit(1);
        }
    }
}