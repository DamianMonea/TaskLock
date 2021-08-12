use levenshtein::levenshtein;
use std::error::Error;
use std::fmt;


#[derive(Debug, Clone)]
pub(crate) struct Command {
    pub(crate) name: String,
    pub(crate) execute: fn() -> Result<bool, CommandExecutionError>,
}

#[derive(Debug)]
pub(crate) struct CommandExecutionError(String);

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an error executing the command: {}", self.0)
    }
}

impl Error for CommandExecutionError {}

#[derive(Debug)]
struct CommandProcessingError(String);

impl fmt::Display for CommandProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an error processing the command: {}", self.0)
    }
}

impl Error for CommandProcessingError {}

pub(crate) fn process_command(input_data: &String, valid_commands: &Vec<Command>) -> Result<Command, Box<dyn Error>>{
    for command in valid_commands {
        if levenshtein(&command.name, input_data) < 2 {
            command.execute;
            return Ok(command.clone());
        }
    }
    Err(Box::new(CommandProcessingError("Something went wrong".into())))
}