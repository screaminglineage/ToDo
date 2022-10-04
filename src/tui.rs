use std::path::Path;

use crate::{
    cli, defaults,
    messages::{error, prompt},
};
use inquire::{list_option::ListOption, validator::ErrorMessage, Select};
use inquire::{
    validator::Validation,
    Editor, MultiSelect, Text,
};
use todo::Task;

// Shows the TUI
pub fn tui(filepath: &Path) {
    println!("Welcome to TUI!");
    // add_task_tui(&filepath);

    loop {
        let items = vec![
            "Add Task",
            "Add Multiple Tasks",
            "Mark as Done",
            "Remove Tasks",
        ];
        let menu = Select::new("Menu", items)
            .prompt_skippable();
        
        match menu {
            Ok(Some("Add Task")) => add_task_tui(false, &filepath),
            Ok(Some("Add Multiple Tasks")) => add_task_tui(true, &filepath),
            Ok(Some("Mark as Done")) => mark_tasks_handler(&filepath),
            Ok(Some("Remove Tasks")) => todo!(),
            Ok(None) => break,
            _ => eprintln!("Error! Main Menu Selection Failed"),
        }
    }
}

// Adds Tasks via TUI
fn add_task_tui(multiple: bool, filepath: &Path) {
    let tasks;

    let validator = |input: &str| {
        if input.contains(defaults::SEPARATOR) {
            Ok(Validation::Invalid(ErrorMessage::Custom(format!(
                "Cannot use the character \"{}\" in a task",
                defaults::SEPARATOR
            ))))
        } else {
            Ok(Validation::Valid)
        }
    };

    if multiple {
        tasks = Editor::new("Type tasks seperated by lines")
            .with_validator(validator)
            .prompt_skippable();

    } else {
        tasks = Text::new("Enter task to add")
        .with_validator(validator)
        .prompt_skippable();
    }

    match tasks {
        Ok(Some(tasks)) => {
            if multiple {
                cli::add_task_handler(tasks.lines().map(|l| l.to_string()).collect(), &filepath)
            } else {
                cli::add_task_handler(vec![tasks], filepath);
            }
            println!("{}", prompt::TASK_ADDED);
        }
        Err(err) => eprintln!("{}: {}", error::ADD_TASK_ERR, err),
        Ok(None) => return (),
    }
}

// Handles Adding Tasks Via the TUI
fn mark_tasks_handler(filepath: &Path) {
    match todo::Task::from_file(&filepath) {
        Ok(tasks) => mark_done_tui(tasks),
        Err(e) => cli::handle_not_found_error(e, error::NO_TASKS, error::MARK_TASK_ERR),
    }
}

// Marks Tasks as Done via TUI (Unfinished)
fn mark_done_tui(tasks: Vec<Task>) {
    let validator = |input: &[ListOption<&Task>]| {
        for opt in input {
            let task = opt.value;
            if task.is_complete() {
                return Ok(Validation::Invalid(
                    "Selected Task(s) has already been marked complete".into(),
                ));
            }
        }
        Ok(Validation::Valid)
    };

    let status = MultiSelect::new("Choose a task: ", tasks)
        .with_validator(validator)
        .prompt_skippable();

    match status {
        Ok(Some(tasks)) => println!("{:?}", tasks),
        Err(e) => eprintln!("Error in Retrieving Tasks: {e}"),
        Ok(None) => return (),
    }
}
