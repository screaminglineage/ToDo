use std::{fs::File, io, path::Path};

use crate::{
    cli, defaults,
    messages::{error, prompt},
};
use inquire::{list_option::ListOption, validator::ErrorMessage, Select};
use inquire::{validator::Validation, Editor, MultiSelect, Text};
use todo::Task;

// Shows the TUI
pub fn tui(filepath: &Path) {
    let mut no_tasks: bool;

    loop {
        let tasks = match todo::get_tasks(&filepath) {
            Ok(tasks) => tasks,
            Err(e) => {
                cli::handle_not_found_error(e, error::NO_TASKS, error::MARK_TASK_ERR);
                return ();
            }
        };

        if tasks.len() == 0 {
            no_tasks = true;
        } else { 
            no_tasks = false
        }

        let items = match no_tasks {
            false => vec![
                "List all Tasks",
                "Add Task",
                "Add Multiple Tasks",
                "Mark as Done",
                "Remove Tasks",
            ],
            true => vec!["Add Task", "Add Multiple Tasks"],
        };

        let menu = Select::new("Menu", items).prompt_skippable();

        match menu {
            Ok(Some("List all Tasks")) => list_tasks_tui(&tasks),
            Ok(Some("Add Task")) => add_task_tui(false, &filepath),
            Ok(Some("Add Multiple Tasks")) => add_task_tui(true, &filepath),
            Ok(Some("Mark as Done")) => tui_mark_handler(tasks, &filepath),
            Ok(Some("Remove Tasks")) => tui_rm_handler(tasks, &filepath),
            Ok(None) => break,
            _ => eprintln!("Error! Main Menu Selection Failed"),
        }
    }
}

// Lists the tasks and then waits for ENTER to be pressed
fn list_tasks_tui(tasks: &[Task]) {
    let mut s = String::with_capacity(1);
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", i+1, task);
    }
    println!("Press ENTER To Continue...");
    if let Err(e) = io::stdin().read_line(&mut s) {
        cli::handle_io_error(e, error::LIST_TASK_ERR);
    }
}

// Adds Tasks via TUI
fn add_task_tui(multiple: bool, filepath: &Path) {
    let status;

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

    // Uses the Text option of inquire for single task addition
    // and the Editor option of inquire (uses the default text editor)
    // for multiple task addition
    if multiple {
        status = Editor::new("Type tasks seperated by lines")
            .with_validator(validator)
            .prompt_skippable();
    } else {
        status = Text::new("Enter task to add: ")
            .with_validator(validator)
            .prompt_skippable();
    }

    match status {
        Ok(Some(tasks)) => {
            if multiple {
                cli::add_task_handler(tasks.lines().map(|l| l.to_string()).collect(), &filepath)
            } else {
                cli::add_task_handler(vec![tasks], &filepath);
            }
            println!("{}", prompt::TASK_ADDED);
        }
        Err(err) => eprintln!("{}: {}", error::ADD_TASK_ERR, err),
        Ok(None) => return (),
    }
}

// Mark Tasks as Complete
fn tui_mark_done(tasks: Vec<Task>, to_mark: Vec<Task>, filepath: &Path) -> io::Result<()> {
    let mut file = File::options().write(true).truncate(true).open(&filepath)?;

    for mut task in tasks {
        if to_mark.contains(&task) {
            task.mark_complete();
        }
        println!("{:?}", task);
        task.write_to_file(&mut file)?;
    }
    Ok(())
}

// Remove Tasks
fn tui_remove_tasks(tasks: Vec<Task>, to_remove: Vec<Task>, filepath: &Path) -> io::Result<()> {
    let mut file = File::options().write(true).truncate(true).open(&filepath)?;

    for task in tasks {
        if !to_remove.contains(&task) {
            task.write_to_file(&mut file)?;
        }
    }
    Ok(())
}

// TUI Handler to mark tasks as complete
fn tui_mark_handler(tasks: Vec<Task>, filepath: &Path) {
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

    let status = MultiSelect::new("Choose Task to Mark as Complete: ", tasks.clone())
        .with_validator(validator)
        .prompt_skippable();

    match status {
        Ok(Some(to_mark)) => {
            if let Ok(()) = tui_mark_done(tasks, to_mark, filepath) {
                println!("{}", prompt::TASK_MARKED);
            } else {
                eprintln!("{}", error::MARK_TASK_ERR);
            }
        }
        Err(e) => eprintln!("Error in Retrieving Tasks: {e}"),
        Ok(None) => return (),
    }
}

// TUI handler to remove tasks
fn tui_rm_handler(tasks: Vec<Task>, filepath: &Path) {
    let status = MultiSelect::new("Select Task to Remove", tasks.clone()).prompt_skippable();

    match status {
        Ok(Some(to_remove)) => {
            if let Ok(()) = tui_remove_tasks(tasks, to_remove, &filepath) {
                println!("{}", prompt::DEL_MARKED);
            } else {
                eprintln!("{}", error::REM_MARK_TASK_ERR);
            }
        }
        Err(e) => eprintln!("Error in Retrieving Tasks: {e}"),
        Ok(None) => return (),
    }
}

