use std::fs;
use log::{debug, error};
use uuid::Uuid;
use crate::atium::common::command_manager::CommandManager;
use crate::atium::common::error::AtiumError;
use crate::atium::common::model::{InfoExtractorResponse, InfoExtractorResponseOutput, InfoFormat, InfoOutputType};
use crate::InfoExtractorRequest;

/// MediaInfo Engine Service for info extraction
pub struct MediaInfoExtractorService {
    command_manager: CommandManager
}

impl MediaInfoExtractorService {
    fn write_info_to_file(&self, output: std::process::Output, out_filepath: String, format: InfoFormat) -> Result<String, &'static str> {
        let ext = match format {
            InfoFormat::Json => ".json",
            InfoFormat::Html => ".html",
            InfoFormat::Xml => ".xml"
        };
        let mut id = out_filepath;
        if id.is_empty() {
            id = Uuid::new_v4().to_string();
        }
        let filename = id;
        let path = filename + ext;

        match fs::write(path.clone(), output.stdout) {
            Ok(_) => {
                debug!("Successfully wrote info to file");
                Ok(path)
            },
            Err(err) => {
                error!("Could not write to file: {}", err);
                Err("could not write to file!")
            }
        }
    }
    fn write_result(&self, execution_result: std::process::Output, request: InfoExtractorRequest, format: InfoFormat) -> Result<InfoExtractorResponse, AtiumError> {
        return match request.output_type.unwrap_or(InfoOutputType::Stdout) {
            InfoOutputType::Stdout => self.command_manager.print_command_output(execution_result.stdout)
                .map(|_| InfoExtractorResponse {
                    output: InfoExtractorResponseOutput {
                        file: None,
                        content: None
                    }
                }),
            InfoOutputType::File => {
                self.write_info_to_file(execution_result, request.output_file.unwrap_or_else(|| String::from("")), format)
                    .map(|output| InfoExtractorResponse {
                        output: InfoExtractorResponseOutput {
                            file: Some(output),
                            content: None
                        }
                    })
                    .map_err(|err_msg| AtiumError::IOError(err_msg.to_string()))
            }
            InfoOutputType::Plain => self.command_manager.get_command_output_as_string(execution_result.stdout)
                .map(|output| InfoExtractorResponse {
                    output: InfoExtractorResponseOutput {
                        file: None,
                        content: Some(output)
                    }
                })
        }
    }
    pub fn new() -> Result<Self, AtiumError> {
        let command_manager =
            CommandManager::new("mediainfo".to_string(), vec!["--Version"])?;

        Ok(Self { command_manager })
    }
    pub fn get_info(&self, request: InfoExtractorRequest) -> Result<InfoExtractorResponse, AtiumError> {

        let binding = request.clone();

        let format = binding.format.unwrap_or(InfoFormat::Json);
        let full = binding.full.unwrap_or(true);

        let mut args: Vec<&str> = Vec::new();

        match format {
            InfoFormat::Json => {
                args.push("--output=JSON");
            }
            InfoFormat::Html => {
                args.push("--output=HTML");
            }
            InfoFormat::Xml => {
                args.push("--output=XML");
            }
        }

        if full {
            args.push("--full");
        }

        args.push(binding.input.as_str());

        return match self.command_manager.execute_with_args(args) {
            Ok(execution_result) => {
                if !execution_result.status.success() {
                    // WARN: MEDIAINFO WRITES ERRORS TO STDOUT
                    self.command_manager.print_command_output(execution_result.stdout)?;
                    return Err(AtiumError::CommandError("Command execution returned ERROR status".to_string()))
                }

                self.write_result(execution_result, request, format)
            }
            Err(_) => Err(AtiumError::CommandError("Could not execute command".to_string()))
        }
    }
}