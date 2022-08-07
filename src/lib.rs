use colored::Colorize;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::process;

const PARSE_ERROR_MESSAGE: &str = "Error in parsing arguments
Make sure that they are in the form 1-5,8,10-12 (without spaces) if marking multiple options";

#[derive(PartialEq, Debug)]
struct Task {
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
    fn from_string(task: &str, seperator: char) -> Task {
        let mut tasks = task.split(seperator);
        let description = match tasks.next() {
            Some(n) => n.to_string(),
            None => panic!("Failed to create Task struct: Couldnt parse name from string"),
        };
        let is_complete = match tasks.next() {
            Some("true") => true,
            Some("false") => false,
            _ => panic!("Failed to create Task struct: Couldnt parse is_complete from string"),
        };

        Task {
            description,
            is_complete,
        }
    }

    // Get a checkbox and the Task
    fn view(&self) -> String {
        if self.is_complete {
            format!(
                "{}{}{} {}",
                "[".blue(),
                "x".red().bold(),
                "]".blue(),
                self.description.green().bold()
            )
        } else {
            format!("{} {}", "[ ]".blue(), self.description.yellow().bold())
        }
    }

    fn set_complete(&mut self) {
        self.is_complete = true;
    }

    // Write a Task to file
    fn write_to_file(&self, file: &mut std::fs::File, separator: char) -> io::Result<()> {
        writeln!(
            file,
            "{}{}{}",
            self.description, separator, self.is_complete
        )?;
        Ok(())
    }
}

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
pub fn add_task(task_name: String, filepath: &String, separator: char) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)?;

    let task = Task::new(task_name);
    task.write_to_file(&mut file, separator)?;
    Ok(())
}

// Displays a list of all tasks
pub fn display_tasks(filepath: &String, separator: char) -> io::Result<()> {
    let tasks_data = fs::read_to_string(filepath)?;

    let mut i: i32 = 1;
    for line in tasks_data.lines() {
        let task = Task::from_string(line, separator);
        println!("{}. {}", i.to_string().blue(), task.view());
        i += 1;
    }
    Ok(())
}

// Deletes a file and renames another temporary file to the former
fn remove_and_rename(original: &String, temp_name: &str) -> io::Result<()> {
    fs::remove_file(&original)?;
    fs::rename(temp_name, &original)?;
    Ok(())
}

// Marks a task as done
pub fn mark_as_done(
    selected_tasks: Vec<u32>,
    filepath: &String,
    separator: char,
) -> io::Result<()> {
    let task_data = fs::read_to_string(&filepath)?;
    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("temp.txt")?;

    let mut i = 1;
    for line in task_data.lines() {
        if selected_tasks.contains(&i) {
            let mut task = Task::from_string(line, separator);
            task.set_complete();
            task.write_to_file(&mut temp_file, separator)?;
        } else {
            writeln!(temp_file, "{line}")?;
        }
        i += 1
    }
    remove_and_rename(&filepath, "temp.txt")?;
    Ok(())
}

// Deletes a specific task
pub fn remove_task(selected_tasks: Vec<u32>, filepath: &String) -> io::Result<()> {
    let task_data = fs::read_to_string(&filepath)?;
    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("temp.txt")?;

    let mut i = 1;
    for line in task_data.lines() {
        if !selected_tasks.contains(&i) {
            writeln!(temp_file, "{line}")?;
        }
        i += 1
    }
    remove_and_rename(&filepath, "temp.txt")?;
    Ok(())
}

// Deletes all tasks from list
pub fn remove_all(filepath: &String) -> io::Result<()> {
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
                eprintln!("{}", PARSE_ERROR_MESSAGE);
                process::exit(1);
            }
        };

        match n.next() {
            Some(Ok(num)) => upper = num,
            None => upper = lower,
            Some(Err(_)) => {
                eprintln!("{}", PARSE_ERROR_MESSAGE);
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
        assert_eq!(
            Task::from_string("Theres another one`false".into(), '`'),
            Task {
                description: "Theres another one".into(),
                is_complete: false
            }
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
