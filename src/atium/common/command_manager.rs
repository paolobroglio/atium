use std::process::{Command, Output};
use crate::atium::common::error::AtiumError;

/// A simple struct that holds the logic needed for managing external commands
pub struct CommandManager {
    command: String
}

impl CommandManager {
    /// Creates a new [`CommandManager`] by trying if the provided command is available on the
    /// executing environment.
    pub fn new(command: String, command_args: Vec<&str>) -> Result<CommandManager, AtiumError> {
        let command_output = Command::new(command.clone())
            .args(command_args)
            .output();

        match command_output {
            Ok(cmd_out) => {
                if !cmd_out.status.success() {
                    return Err(AtiumError::CommandError("Command execution returned ERROR status".to_string()))
                }

                Ok(CommandManager { command })
            }
            Err(_) =>
                Err(AtiumError::CommandError("error when executing command".to_string()))
        }
    }
    /// Prints Command Output to stdout
    pub fn print_command_output(&self, output: Vec<u8>) -> Result<(), AtiumError> {
        match String::from_utf8(output) {
            Ok(content) => {
                content.lines().for_each(|x| println!("{:?}", x));
                Ok(())
            },
            Err(_) => Err(AtiumError::IOError("error when writing to stdout".to_string())),
        }
    }
    pub fn execute_with_args(&self, args: Vec<&str>) -> Result<Output, AtiumError> {
        let command = Command::new(self.command.clone())
            .args(args)
            .output();

        // todo: add with logging facility
        // println!();
        // let all_args: Vec<&OsStr> = command_with_args.get_args().collect();
        // all_args.iter().for_each(|a| print!(" {} ", a.to_str().unwrap_or("")));
        // println!();

        match command {
            Ok(result) => Ok(result),
            Err(_) => Err(AtiumError::CommandError("error when executing command".to_string())),
        }
    }
}

