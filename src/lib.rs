use colored::Colorize;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::ops::Range;

pub mod messages;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    text: String,
    status: TaskStatus,
}

impl Task {
    pub fn new(task: &str) -> Self {
        Self {
            text: task.to_string(),
            status: TaskStatus::Incomplete,
        }
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn is_complete(&self) -> bool {
        self.status == TaskStatus::Complete
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_complete() {
            write!(
                f,
                "{}{}{} {}",
                "[".bold(),
                "x".red().bold(),
                "]".bold(),
                self.text.green().bold()
            )
        } else {
            write!(f, "{} {}", "[ ]".bold(), self.text.yellow().bold())
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Incomplete,
    Complete,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ParsePatternError,
}

// Removes all invalid indexes from the Range and converts it into a vector of 0-indexed indexes
fn validate_range(ranges: Vec<Range<u32>>, tasks: &Vec<Task>) -> Vec<u32> {
    ranges
        .into_iter()
        .filter(|range| {
            (range.start as usize) <= tasks.len() && ((range.end as usize) - 1) <= tasks.len()
        })
        .flat_map(|range| (range.start - 1)..(range.end - 1))
        .collect()
}

pub fn add_task(tasks: &mut Vec<Task>, text: &str) {
    tasks.push(Task::new(text));
}

// Changes the status of specific tasks
pub fn change_task_status(
    tasks: &mut Vec<Task>,
    ranges_to_mark: Vec<Range<u32>>,
    new_status: TaskStatus,
) {
    let indexes = validate_range(ranges_to_mark, tasks);

    for index in indexes {
        if let Some(t) = tasks.get_mut(index as usize) {
            t.set_status(new_status);
        }
    }
}

// Removes tasks based on the provided range
pub fn remove_tasks(tasks: &mut Vec<Task>, ranges_to_remove: Vec<Range<u32>>) {
    let mut indexes = validate_range(ranges_to_remove, tasks);
    indexes.sort_by(|a, b| b.cmp(a));

    for index in indexes {
        tasks.remove(index as usize);
    }
}

// Removes all tasks marked complete
pub fn remove_completed_tasks(tasks: &mut Vec<Task>) {
    tasks.retain(|task| !task.is_complete());
}

// Removes all tasks
pub fn remove_all(tasks: &mut Vec<Task>) {
    tasks.clear();
}

// Displays a list of all tasks
pub fn display_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("{}", messages::NO_TASKS_TO_DISPLAY);
    }
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", format!("{}", i + 1).bold(), task);
    }
}

// Displays a prompt to the user and returns their input
pub fn take_input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

// Parses a user entered pattern like "1-6,13,7-9" into a Vec of Ranges
pub fn parse_pattern(pattern: &str) -> Result<Vec<Range<u32>>, Error> {
    let mut tasks = Vec::new();

    for nums in pattern.split(',') {
        let mut nums_it = nums.split('-').map(|s| s.parse::<u32>());

        let start = match nums_it.next() {
            Some(Ok(num)) => num,
            _ => return Err(Error::ParsePatternError),
        };

        let end = match nums_it.next() {
            Some(Ok(num)) => num,
            None => start,
            Some(Err(_)) => return Err(Error::ParsePatternError),
        };

        tasks.push(Range {
            start,
            end: end + 1,
        })
    }
    Ok(tasks)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_stuff() -> Vec<Task> {
        vec![
            Task::new("Task 1"),
            Task::new("Task 2"),
            Task::new("Task 3"),
            Task::new("Task 4"),
        ]
    }

    #[test]
    fn pattern_parser_simple() {
        assert_eq!(parse_pattern("1,5,7"), Ok(vec![1..2, 5..6, 7..8]));
    }

    #[test]
    fn pattern_parser_complex() {
        assert_eq!(
            parse_pattern("1-6,13-17,7-9,14"),
            Ok(vec![1..7, 13..18, 7..10, 14..15])
        );
    }

    #[test]
    fn pattern_parser_secondary_missing() {
        assert_eq!(parse_pattern("1-"), Err(Error::ParsePatternError));
    }

    #[test]
    fn removing_completed_tasks() {
        let mut tasks = setup_stuff();
        tasks[0].set_status(TaskStatus::Complete);
        tasks[1].set_status(TaskStatus::Complete);
        remove_completed_tasks(&mut tasks);

        assert_eq!(
            tasks,
            vec![
                Task {
                    text: "Task 3".into(),
                    status: TaskStatus::Incomplete
                },
                Task {
                    text: "Task 4".into(),
                    status: TaskStatus::Incomplete
                },
            ]
        )
    }

    #[test]
    fn removing_selected_tasks() {
        let mut tasks = setup_stuff();
        let marked = parse_pattern("1-2,4").unwrap();
        remove_tasks(&mut tasks, marked);
        assert_eq!(
            tasks,
            vec![Task {
                text: "Task 3".into(),
                status: TaskStatus::Incomplete
            }]
        );
    }

    #[test]
    fn validating_range() {
        let tasks = setup_stuff();
        let marked = parse_pattern("1-5,1-3,5-10").unwrap();
        assert_eq!(validate_range(marked, &tasks), vec![0, 1, 2])
    }

    #[test]
    fn marking_selected_tasks() {
        let mut tasks = setup_stuff();
        let marked = parse_pattern("1-2,4,9-9999").unwrap();
        change_task_status(&mut tasks, marked, TaskStatus::Complete);
        assert_eq!(
            tasks,
            vec![
                Task {
                    text: "Task 1".into(),
                    status: TaskStatus::Complete
                },
                Task {
                    text: "Task 2".into(),
                    status: TaskStatus::Complete
                },
                Task {
                    text: "Task 3".into(),
                    status: TaskStatus::Incomplete
                },
                Task {
                    text: "Task 4".into(),
                    status: TaskStatus::Complete
                },
            ]
        );
    }
    #[test]
    fn removing_all() {
        let mut tasks = setup_stuff();
        remove_all(&mut tasks);
        assert!(tasks.is_empty());
    }

    #[test]
    fn adding_new_task() {
        let mut tasks = setup_stuff();
        add_task(&mut tasks, "This is a New Task!");
        assert_eq!(
            tasks,
            vec![
                Task {
                    text: "Task 1".into(),
                    status: TaskStatus::Incomplete
                },
                Task {
                    text: "Task 2".into(),
                    status: TaskStatus::Incomplete
                },
                Task {
                    text: "Task 3".into(),
                    status: TaskStatus::Incomplete
                },
                Task {
                    text: "Task 4".into(),
                    status: TaskStatus::Incomplete
                },
                Task {
                    text: "This is a New Task!".into(),
                    status: TaskStatus::Incomplete
                },
            ]
        );
    }
}
