#![allow(unused)]

pub mod error {
    pub const PATTERN_PARSE_ERR: &'static str = "Error in parsing arguments
Make sure that they are in the form 1-5,8,10-12 (without spaces) if marking multiple options";

    pub const TASK_PARSE_ERR_NAME: &'static str =
        "Failed to create struct 'Task' from string: Couldnt parse name from string";
    pub const TASK_PARSE_ERR_CMPL: &'static str =
        "Failed to create struct 'Task' from string: Couldnt parse is_complete from string";
    pub const TASK_PARSE_ERR_FILE: &'static str =
        "Error in Creating Task from File. Failed to Open File";

    pub const NO_TASKS: &'static str = "No Saved Tasks Found!";
    pub const NO_TASKS_DISPL: &'static str = "No Tasks to Display!";
    pub const ADD_TASK_ERR: &'static str = "Error in Adding Task(s)";
    pub const MARK_TASK_ERR: &'static str = "Error in Marking Task(s)";
    pub const REM_TASK_ERR: &'static str = "Error in Removing Task(s)";
    pub const REM_MARK_TASK_ERR: &'static str = "Error in Removing Marked Task(s)";
    pub const DEL_TASK_ERR: &'static str = "Error in Deleting all Tasks";
    pub const LIST_TASK_ERR: &'static str = "Error in Displaying Tasks";
    pub const ENV_VAR_ERR: &'static str = "Couldnt Get Tasks File Path from Environment Variable.
Make sure it's set to file and not a folder.";
}

pub mod prompt {
    pub const TASK_ADDED: &'static str = "Task(s) Added";
    pub const TASK_MARKED: &'static str = "Task(s) Marked as Done";
    pub const DELETE_ALL: &'static str = "Do you want to delete all saved tasks (y/N): ";
    pub const DEL_MARKED: &'static str = "All Marked Task(s) Removed";
    pub const DEL_ALL: &'static str = "All Saved Tasks Deleted";
    pub const DEL_CANCEL: &'static str = "Tasks Left Unchanged";
}
