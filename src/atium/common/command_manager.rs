use std::process::Command;

/// A simple struct that holds the logic needed for managing external commands
pub struct CommandManager {
    command: String
}

impl CommandManager {
    /// Creates a new [`CommandManager`] by trying if the provided command is available on the
    /// executing environment.
    pub fn new(command: String, command_args: Vec<&str>) -> Result<CommandManager, &'static str> {
        let command_output = Command::new(command.clone())
            .args(command_args)
            .output();

        match command_output {
            Ok(cmd_out) => {
                if !cmd_out.status.success() {
                    return Err("Execution of command failed");
                }

                Ok(CommandManager { command })
            }
            Err(_) =>
                Err("Execution of command failed")
        }
    }
    /// Prints Command Output to stdout
    pub fn print_command_output(&self, output: Vec<u8>) -> Result<(), &'static str> {
        match String::from_utf8(output) {
            Ok(content) => {
                content.lines().for_each(|x| println!("{:?}", x));
                Ok(())
            },
            Err(_) => Err("error when writing to stdout"),
        }
    }
}

