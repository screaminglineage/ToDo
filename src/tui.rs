use std::path::Path;

use crate::{
    cli, defaults,
    messages::{error, prompt},
};
use inquire::{list_option::ListOption, validator::ErrorMessage, Select};
use inquire::{
    ui::{IndexPrefix, RenderConfig},
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
            "Exit",
        ];
        let menu = Select::new("Menu", items).prompt();

        match menu {
            Ok("Add Task") => add_task_tui(&filepath),
            Ok("Add Multiple Tasks") => add_mult_task_tui(&filepath),
            Ok("Mark as Done") => mark_tasks_handler(&filepath),
            Ok("Remove Tasks") => todo!(),
            Ok("Exit") => break,
            _ => eprintln!("Error! Main Menu Selection Failed"),
        }
    }
}

// Adds Tasks via TUI
fn add_task_tui(filepath: &Path) {
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

    let task = Text::new("Enter task to add: ")
        .with_validator(validator)
        .prompt();

    match task {
        Ok(task) => {
            cli::add_task_handler(vec![task], filepath);
            println!("{}", prompt::TASK_ADDED);
        }

        Err(err) => eprintln!("{}: {}", error::ADD_TASK_ERR, err),
    }
}

// Adds Multiple Tasks via TUI
fn add_mult_task_tui(filepath: &Path) {
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

    let tasks = Editor::new("Type tasks seperated by lines: ")
        .with_validator(validator)
        .prompt();

    match tasks {
        Ok(tasks) => {
            cli::add_task_handler(tasks.lines().map(|l| l.to_string()).collect(), &filepath)
        }
        Err(err) => eprintln!("{}: {}", error::ADD_TASK_ERR, err),
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

    let render_config = RenderConfig::default().with_option_index_prefix(IndexPrefix::Simple);

    let status = MultiSelect::new("Choose a task: ", tasks)
        .with_render_config(render_config)
        .with_validator(validator)
        .prompt();

    match status {
        Ok(tasks) => println!("{:?}", tasks),
        Err(e) => eprintln!("Error in Retrieving Tasks: {e}"),
    }
}
