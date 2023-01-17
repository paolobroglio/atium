use log::{info};
use crate::atium::common::command_manager::CommandManager;
use crate::atium::common::error::AtiumError;
use crate::atium::common::model::{ThumbnailRequest, ThumbnailResponse};
use crate::atium::utility::helper::compute_output_file;


pub struct FFMPEGThumbnailService {
    command_manager: CommandManager
}

impl FFMPEGThumbnailService{
    fn build_args(&self, input_file: String, output_file: String, timestamp: String) -> Vec<String> {
        vec![
            String::from("-i"),
            input_file,
            String::from("-ss"),
            timestamp,
            String::from("-vframes"),
            String::from("1"),
            output_file,
        ]
    }
    fn build_output_from_input_path(&self, input_file: String) -> String {
        let mut out = input_file;
        out.push_str(".jpeg");

        out
    }
    /// Extracts a thumbnail and returns a [`ThumbnailResponse`]
    pub fn extract_thumbnail(&self, thumbnail_request: ThumbnailRequest) -> Result<ThumbnailResponse, AtiumError> {
        let input_file = thumbnail_request.input_file.expect("INPUT_FILE Cannot be empty");

        let mut output_file = thumbnail_request.output_file
            .unwrap_or_else(|| self.build_output_from_input_path(input_file.clone()));

        output_file = compute_output_file(&output_file);

        let timestamp = thumbnail_request.timestamp
            .unwrap_or_else(|| String::from("00:00:01.000"));

        let args = self.build_args(input_file, output_file.clone(), timestamp);

        match self.command_manager.execute_with_args(args.iter().map(AsRef::as_ref).collect()) {
            Ok(result) => {
                if !result.status.success() {
                    self.command_manager.print_command_output(result.stderr)?;
                    return Err(AtiumError::ConversionError("Execution of command returned ERROR".to_string()))
                }
                info!("Thumbnail extracted at path [{}]", output_file);
                Ok(ThumbnailResponse{ output: output_file })
            }
            Err(err) => Err(err)
        }
    }
    /// Constructs a new instance of [`FFMPEGThumbnailService`] by loading and checking `ffmpeg` availability
    pub fn new() -> Result<Self, AtiumError> {
        let command_manager =
            CommandManager::new("ffmpeg".to_string(), vec!["-version"])?;

        Ok(Self { command_manager })
    }
}
