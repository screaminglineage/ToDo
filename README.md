# ToDo
A simple command line TODO utility

## Contents
  1. [Introduction](#introduction)
  2. [Usage](#usage)
      - [Options](#options)
      - [Examples](#examples)
      
---

## Introduction
ToDo is a simple CLI based TODO utility which lets you add tasks to a list, mark them done in bulk and remove them from the list entirely.

## Usage
All the added tasks are listed out by default just by running the program without any options 

```
todo 1.0.0
Add tasks to a TODO list and then mark them done or remove when required

USAGE:
    todo [OPTIONS] [...]

ARGS:
    <...>    Lists all tasks when no options are given

OPTIONS:
    -a, --add <ADD>            Add a new task
    -d, --delete               Delete all tasks
    -h, --help                 Print help information
    -r, --remove <TASKS>       Remove a specific task. A pattern like 1-5,8,10-12 (without spaces)
                               can also be used to remove multiple tasks at once
    -V, --version              Print version information
    -x, --mark-done <TASKS>    Mark a task as complete. A pattern like 1-5,8,10-12 (without spaces)
                               can also be used to mark multiple tasks at once
```

### Options
 - `-a`, `--add` - Add a new task to the list
 - `-x`, `--mark-done` - Mark specific tasks as done by specifying them separated by commas (A '**-**' can be used to denote ranges)
 - `-r`, `--remove` - Delete a specific tasks by specifying them (same pattern as for marking them as done)
 - `-d`, `--delete` - Delete all saved tasks
 
 ### Examples
  - `todo -a "Task 1"` - Add a new task, **Task 1** to the list (Tasks must be enclosed within quotes if they include spaces)
  - `todo -x 1-5,14,7,10-12` - Mark tasks **1 to 5**, **7**, **10 to 12**, and **14** as done in the list (Note that theres no space after the commas in the command)
  - `todo -r 1-3,8,10` - Remove tasks **1 to 3**, **8**, and **10** from the list (Same pattern as for marking tasks done)
  - `todo -d` - Delete all saved tasks (Gives a secondary warning to prevent accidental deletion)
