use std::{
    env, error,
    fs::{self, File},
    io::ErrorKind,
};

use todo::Task;
mod cli;

pub const FILEPATH_ENV_VAR: &str = "RTODO_FILE_PATH";
pub const DEFAULT_TASKS_FILE: &str = "todo_tasks.json";

fn main() -> Result<(), Box<dyn error::Error>> {
    let file_path = match env::var(FILEPATH_ENV_VAR) {
        Ok(path) => path,
        Err(_) => DEFAULT_TASKS_FILE.to_string(),
    };

    // Tries to read the JSON file and returns an empty Vec as string if not found
    let json_data = match fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(e) if e.kind() == ErrorKind::NotFound => "[]".to_string(),
        Err(e) => return Err(Box::new(e)),
    };

    // Tries to deserialize the JSON data into a Vec of tasks and displays errors if any
    let mut tasks: Vec<Task> = match serde_json::from_str(&json_data) {
        Ok(t) => t,
        Err(e) => {
            if e.is_syntax() {
                eprintln!("Error: invalid JSON in file - '{file_path}'\nReason: {e}");
                return Ok(());
            }
            return Err(Box::new(e));
        }
    };

    if let Err(todo::Error::ParsePatternError) = cli::cli_run(&mut tasks) {
        eprintln!("{}", todo::messages::PATTERN_PARSE_ERR);
    }

    let file = File::create(file_path)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}
