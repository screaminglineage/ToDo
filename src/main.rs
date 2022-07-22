use clap::{ArgGroup, Parser};
use std::fs;
use std::process;
use std::io;

use todo;

const FILENAME: &str = "tasks.txt";
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
    #[clap(long="mark-done", short='x', value_parser=check_marked, value_name="TASK NUMBER")]
    mark: Option<u32>,

    /// Remove all tasks
    #[clap(long, short, action, value_parser)]
    remove: bool,
}

fn main() {
    let cli = Cli::parse();

    // Adding a task
    if let Some(task) = cli.task {
        println!("Adding - {task}"); // testing code
        if let Err(e) = todo::add_task(task, FILENAME, SEPARATOR) {
            handle_error(e, "Error in Adding Task");
        };
    }

    // Listing all saved tasks
    if cli.list {
        println!("Listing all tasks"); // testing code
        if let Err(e) = todo::display_tasks(FILENAME, SEPARATOR) {
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("No tasks to display!");
                process::exit(1);
            }
        }
    }

    // Marking specific task as done
    if let Some(num) = cli.mark {
        println!("Marking task {num} as done"); //testing code
        if let Err(e) = todo::mark_as_done(num, FILENAME, SEPARATOR) {
            handle_error(e, "Error in Marking Task Done");
        }
    }

    // Removing all saved tasks
    if cli.remove {
        let choice = todo::take_input(
            "Do you want to remove all saved tasks (y/n): "
        );
        match choice.to_lowercase().trim() {
            "y" => {
                println!("Removing all saved tasks"); 
                if let Err(e) = todo::remove_all(FILENAME) {
                    handle_error(e, "Error in Deleting Tasks");
                }
            },
            _ => {
                println!("No tasks deleted"); 
                process::exit(0);
            }
        }
    }
}


fn handle_error(error: io::Error, desc: &str) {
    eprintln!("{desc} - {error}");
    process::exit(1);
}



fn check_marked(n: &str) -> Result<u32, String> {
    let task_data = fs::read_to_string(FILENAME).expect("Failed to read file");
    let count = task_data.lines().count();

    let num: usize = n
        .parse()
        .map_err(|_| format!("{n} is not a valid number"))?;

    if num <= count && num > 0 {
        Ok(num as u32)
    } else {
        Err("Number out of range!".to_string())
    }
}
