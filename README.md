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
todo 1.1.0
Add tasks to a TODO list and then mark them done or remove when required

USAGE:
    todo [OPTIONS] [...]
 
OPTIONS:
    -a, --add <NEW-TASKS>      Add new tasks separated by commas (without any spaces in between)
    -d, --delete               Delete all tasks
    -h, --help                 Print help information
    -r, --remove <TASKS>       Remove a specific task. A pattern like 1-5,8,10-12 (without spaces)
                               can also be used to remove multiple tasks at once
    -R, --remove-marked        Remove all tasks marked as done
    -V, --version              Print version information
    -x, --mark-done <TASKS>    Mark a task as complete. A pattern like 1-5,8,10-12 (without spaces)
                               can also be used to mark multiple tasks at once
```

### Options
 - `-a`, `--add` - Add a new tasks to the list (Multiple tasks can be added as comma separated values)
 - `-x`, `--mark-done` - Mark specific tasks as done by specifying them separated by commas (A '**-**' can be used to denote ranges)
 - `-r`, `--remove` - Delete a specific tasks by specifying them (same pattern as for marking them as done)
 - `-R`, `--remove-marked` - Delete all the tasks which have been marked complete
 - `-d`, `--delete` - Delete all saved tasks
 
 ### Examples
  - `todo -a "Task 1","Task 2","Task 3"` - Adds **Task 1**, **Task 2**, and **Task 3** to the list. (Tasks must be enclosed within quotes if they include spaces. There also shouldnt be any spaces after each comma between the tasks.)
  - `todo -x 1-5,14,7,10-12` - Mark tasks **1 to 5**, **7**, **10 to 12**, and **14** as done in the list (Note that theres no space after the commas in the command)
  - `todo -r 1-3,8,10` - Remove tasks **1 to 3**, **8**, and **10** from the list (Same pattern as for marking tasks done)
  - `todo -R` - Remove all tasks in the list which have been marked as done
  - `todo -d` - Delete all saved tasks (Gives a secondary warning to prevent accidental deletion)
