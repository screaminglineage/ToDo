#![allow(unused)]

pub mod error {
    pub const PATTERN_PARSE_ERR: &'static str = "Error in parsing arguments
Make sure that they are in the form 1-5,8,10-12 (without spaces) if marking multiple options";

    pub const TASK_NAME_PARSE_ERR: &'static str =
        "Failed to create struct 'Task' from string: Couldnt parse name from string";
    pub const TASK_MARKED_PARSE_ERR: &'static str =
        "Failed to create struct 'Task' from string: Couldnt parse is_complete from string";

    pub const NO_TASKS: &'static str = "No Saved Tasks Found!";
    pub const NO_TASKS_DISPL: &'static str = "No Tasks to Display!";
    pub const ADD_TASK_ERR: &'static str = "Error in Adding Task";
    pub const MARK_TASK_ERR: &'static str = "Error in Marking Tasks";
    pub const REM_TASK_ERR: &'static str = "Error in Removing Tasks";
    pub const REM_MARK_TASK_ERR: &'static str = "Error in Removing Marked Tasks";
    pub const DEL_TASK_ERR: &'static str = "Error in Deleting Tasks";
    pub const LIST_TASK_ERR: &'static str = "Error in Displaying Tasks";
    pub const ENV_VAR_ERR: &'static str = "Couldnt Get Tasks Filepath from Environment Variable.
Make sure it's set to an actual file.";
}

pub mod prompt {
    pub const TASK_ADDED: &'static str = "Task(s) Added";
    pub const DELETE_ALL: &'static str = "Do you want to delete all saved tasks (y/N): ";
    pub const DEL_MARKED: &'static str = "Marked Tasks Removed";
    pub const DEL_ALL: &'static str = "All Saved Tasks Deleted";
    pub const DEL_CANCEL: &'static str = "Tasks Left Unchanged";
}
