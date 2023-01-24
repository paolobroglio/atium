use chrono::{NaiveTime};
use log::{debug, error, info, warn};
use crate::atium::common::analysis_helper::{compute_output_file, MediaInfoJsonLoader};
use crate::atium::common::command_manager::CommandManager;
use crate::atium::common::error::AtiumError;
use crate::atium::common::model::{InfoFormat, InfoOutputType, ThumbnailRequest, ThumbnailResponse};
use crate::{InfoExtractorRequest, MediaInfoExtractorService};


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
    fn get_source_duration(&self, input_file: String) -> Result<String, AtiumError> {
        let service = MediaInfoExtractorService::new()?;
        let request = InfoExtractorRequest {
            input: input_file,
            format: Some(InfoFormat::Json),
            full: None,
            output_file: None,
            output_type: Some(InfoOutputType::Plain)
        };

        let response = service.get_info(request)?
            .output
            .content
            .unwrap_or_else(|| String::from(""));

        let media_info_loader = MediaInfoJsonLoader{};
        match media_info_loader.load_json_from_string(&response) {
            Ok(output) => {
                debug!("Analysis done!");

                output.extract_field_from_track(0, &"Duration_String3".to_string())
                    .map(|duration_field| {
                        let split = duration_field.split('.').collect::<Vec<_>>();
                        let duration_in_secs = split.get(0)
                            .map(|s| s.to_string())
                            .unwrap_or("00:00:01".to_string());

                        debug!("Duration in hh:mm:ss format: [{}]", duration_in_secs);

                        duration_in_secs
                    })
            }
            Err(err) => {
                error!("Could not analyze input file: {}", err);
                Err(err)
            }
        }
    }
    fn compute_timestamp(&self, input_file: String, thumbnail_request: ThumbnailRequest) -> Result<String, AtiumError> {
        debug!("Computing timestamp for thumbnail extraction");
        let binding_req_ts = thumbnail_request.timestamp.unwrap_or_else(|| String::from("00:00:01"));
        let source_duration = self.get_source_duration(input_file)?;
        let req_time = NaiveTime::parse_from_str(binding_req_ts.as_str(), "%H:%M:%S")
            .map_err(|err| {
                warn!("Error parsing requested timestamp: {}", err);
                AtiumError::IOError("An error occurred when parsing requested timestamp".to_string())
            })?;
        let source_duration_time = NaiveTime::parse_from_str(source_duration.as_str(), "%H:%M:%S")
            .map_err(|err| {
                warn!("Error parsing duration timestamp: {}", err);
                AtiumError::IOError("An error occurred when parsing duration timestamp".to_string())
            })?;

        if req_time.lt(&source_duration_time) {
            Ok(binding_req_ts)
        } else {
            debug!("Requested timestamp is greater than the input duration");
            debug!("Thumbnail extraction timestamp will be set to 00:00:00.000");

            Ok("00:00:00.000".to_string())
        }
    }
    /// Extracts a thumbnail and returns a [`ThumbnailResponse`]
    pub fn extract_thumbnail(&self, thumbnail_request: ThumbnailRequest) -> Result<ThumbnailResponse, AtiumError> {
        let input_file = thumbnail_request.clone().input_file.expect("INPUT_FILE Cannot be empty");

        let mut output_file = thumbnail_request.clone().output_file
            .unwrap_or_else(|| self.build_output_from_input_path(input_file.clone()));

        output_file = compute_output_file(&output_file, "jpeg")?;

        let timestamp = self.compute_timestamp(input_file.clone(), thumbnail_request)?;

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