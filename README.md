# ToDo
A simple command line TODO utility

## Contents
  1. [Introduction](#introduction)
  2. [Changing the Task Data Location](#changing-the-task-data-location)
  3. [Usage](#usage)
      - [Options](#options)
      - [Examples](#examples)
      
---

## Introduction
ToDo is a simple CLI based TODO utility which lets you add tasks to a list, mark them done in bulk and remove them from the list entirely.
Also has a simple TUI launched using the `tui` subcommand

## Changing the Task Data Location
The task data is saved in a file called **todo_tasks.txt** (in the current working directory) by default. However an environment variable, **RTODO_FILE_PATH**, can be set to any desired value to specify a custom path for saving the task data. 

## Usage
All the added tasks are listed out by default just by running the program without any options 

```
Add tasks to a TODO list and then mark them done or remove when required

Usage: todo [OPTIONS] [COMMAND]

Commands:
  tui   Launch ToDo in TUI mode
  help  Print this message or the help of the given subcommand(s)

Options:
  -a, --add <NEW-TASK(S)>    Add new tasks separated by commas (without any spaces in between)
  -x, --mark-done <TASK(S)>  Mark a task as complete
  -r, --remove <TASK(S)>     Remove a specific task
  -R, --remove-marked        Remove all tasks which have been marked as complete
  -D, --delete-all           Delete all tasks
  -h, --help                 Print help information (use `--help` for more detail)
  -V, --version              Print version information
```

### Options
 - `-a`, `--add` - Add a new tasks to the list (Multiple tasks can be added as comma separated values)
 - `-x`, `--mark-done` - Mark specific tasks as done by specifying them separated by commas (A '**-**' can be used to denote ranges)
 - `-r`, `--remove` - Delete a specific tasks by specifying them (same pattern as for marking them as done)
 - `-R`, `--remove-marked` - Delete all the tasks which have been marked complete
 - `-D`, `--delete-all` - Delete all saved tasks
 
 ### Examples
  - `todo tui` - Launches ToDo in TUI (Terminal User Interface) mode  
  - `todo -a "Task 1","Task 2","Task 3"` - Adds **Task 1**, **Task 2**, and **Task 3** to the list. (Tasks must be enclosed within quotes if they include spaces. There also shouldnt be any spaces after each comma between the tasks.)
  - `todo -x 1-5,14,7,10-12` - Mark tasks **1 to 5**, **7**, **10 to 12**, and **14** as done in the list (Note that theres no space after the commas in the command)
  - `todo -r 1-3,8,10` - Remove tasks **1 to 3**, **8**, and **10** from the list (Same pattern as for marking tasks done)
  - `todo -R` - Remove all tasks in the list which have been marked as done
  - `todo -D` - Delete all saved tasks (Gives a secondary warning to prevent accidental deletion)
