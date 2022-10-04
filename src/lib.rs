use colored::Colorize;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process;

mod defaults;
mod messages;
use messages::error;

#[derive(PartialEq, Debug)]
pub struct Task {
    description: String,
    is_complete: bool,
}

impl Task {
    fn new(name: String) -> Task {
        Task {
            description: name,
            is_complete: false,
        }
    }

    // Create Task from a string separated by a character
    pub fn from_string(task: &str) -> Task {
        let mut tasks = task.split(defaults::SEPARATOR);
        let description = match tasks.next() {
            Some(n) => n.to_string(),
            None => panic!("{}", error::TASK_PARSE_ERR_NAME),
        };
        let is_complete = match tasks.next() {
            Some("true") => true,
            Some("false") => false,
            _ => panic!("{}", error::TASK_PARSE_ERR_CMPL),
        };

        Task {
            description,
            is_complete,
        }
    }

    // Create a Task from a given filepath
    pub fn from_file(filepath: &Path) -> io::Result<Vec<Task>> {
        let contents = fs::read_to_string(filepath)?;
        Ok(contents.lines().map(|l| Task::from_string(l)).collect())
    }

    fn set_complete(&mut self) {
        self.is_complete = true;
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    // Write a Task to file
    fn write_to_file(&self, file: &mut std::fs::File) -> io::Result<()> {
        writeln!(
            file,
            "{}{}{}",
            self.description,
            defaults::SEPARATOR,
            self.is_complete
        )?;
        Ok(())
    }
}

// Displays the Task along with a checkbox to denote
// if it is complete or not. The colour of the Task is also
// set depending on whether it is complete or not.
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_complete {
            write!(f, "{}", self.description.green().bold())
        } else {
            write!(f, "{}", self.description.yellow().bold())
        }
    }
}

// TODO: Change this to return a result and handle both the error cases in main.rs
// Displays a prompt to the user and returns their input
pub fn take_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush buffer");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

// Adds a new task to the list
pub fn add_task(task_name: String, filepath: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)?;

    let task = Task::new(task_name);
    task.write_to_file(&mut file)?;
    Ok(())
}

// Displays a list of all tasks
pub fn display_tasks(filepath: &Path) -> io::Result<()> {
    let tasks_data = fs::read_to_string(filepath)?;

    for (i, line) in tasks_data.lines().enumerate() {
        let task = Task::from_string(line);
        print!("{}. ", (i + 1).to_string().blue());

        if task.is_complete() {
            print!("{}{}{} ", "[".blue(), "x".red().bold(), "]".blue(),);
        } else {
            print!("{} ", "[ ]".blue());
        }
        println!("{}", task);
    }
    Ok(())
}

// Deletes a file and renames another temporary file to the former
fn remove_and_rename(original: &Path, temp_file: &Path) -> io::Result<()> {
    fs::remove_file(&original)?;
    fs::rename(temp_file, &original)?;
    Ok(())
}

// Marks a task as done
pub fn mark_as_done(selected_tasks: Vec<u32>, filepath: &Path, temp_path: &Path) -> io::Result<()> {
    let task_data = fs::read_to_string(&filepath)?;
    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(temp_path)?;

    let mut i = 1;
    for line in task_data.lines() {
        if selected_tasks.contains(&i) {
            let mut task = Task::from_string(line);
            task.set_complete();
            task.write_to_file(&mut temp_file)?;
        } else {
            writeln!(temp_file, "{line}")?;
        }
        i += 1
    }
    remove_and_rename(filepath, temp_path)?;
    Ok(())
}

// Removes a specific task
pub fn remove_task(selected_tasks: Vec<u32>, filepath: &Path, temp_path: &Path) -> io::Result<()> {
    let task_data = fs::read_to_string(&filepath)?;
    let mut temp_file = File::create(temp_path)?;

    let mut i = 1;
    for line in task_data.lines() {
        if !selected_tasks.contains(&i) {
            writeln!(temp_file, "{line}")?;
        }
        i += 1
    }
    remove_and_rename(filepath, temp_path)?;
    Ok(())
}

// Removes all tasks marked as done
pub fn remove_marked(filepath: &Path, temp_path: &Path) -> io::Result<()> {
    let task_data = fs::read_to_string(&filepath)?;
    let mut temp_file = File::create(temp_path)?;

    for line in task_data.lines() {
        let task = Task::from_string(line);
        if !task.is_complete {
            writeln!(temp_file, "{line}")?;
        }
    }
    remove_and_rename(filepath, temp_path)?;
    Ok(())
}

// Deletes all tasks from list
pub fn remove_all(filepath: &Path) -> io::Result<()> {
    fs::remove_file(filepath)?;
    Ok(())
}

// Parses a user entered pattern like "1-6,13,7-9" into [1,2,3,4,5,6,13,7,8,9]
pub fn parse_pattern(pattern: String) -> Vec<u32> {
    let mut tasks = Vec::new();
    for num in pattern.split(",") {
        let mut n = num.split("-").map(|s| s.parse::<u32>());

        let lower: u32;
        let upper: u32;

        match n.next() {
            Some(Ok(num)) => lower = num,
            _ => {
                eprintln!("{}", error::PATTERN_PARSE_ERR);
                process::exit(1);
            }
        };

        match n.next() {
            Some(Ok(num)) => upper = num,
            None => upper = lower,
            Some(Err(_)) => {
                eprintln!("{}", error::PATTERN_PARSE_ERR);
                process::exit(1);
            }
        };
        for i in lower..upper + 1 {
            tasks.push(i);
        }
    }
    tasks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task_from_string() {
        let task = Task::from_string("Theres another one`false".into());
        assert_eq!(
            format!("{}", task),
            format!("{} {}", "[ ]".blue(), task.description.yellow().bold())
        );
    }

    #[test]
    fn pattern_parser_simple() {
        assert_eq!(parse_pattern("1,5,7".into()), [1, 5, 7]);
    }

    #[test]
    fn pattern_parser_complex() {
        assert_eq!(
            parse_pattern("1-6,13-17,7-9,14".into()),
            [1, 2, 3, 4, 5, 6, 13, 14, 15, 16, 17, 7, 8, 9, 14]
        );
    }
}
