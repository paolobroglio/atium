use std::fs;

use log::{debug, error, warn};
use uuid::Uuid;

use crate::{InfoExtractorRequest, MediaInfoExtractorService, ThumbnailRequest};
use crate::atium::common::analysis_helper::{AnalysisOutput, compute_output_file, MediaInfoJsonLoader};

use crate::atium::common::command_manager::CommandManager;
use crate::atium::common::error::AtiumError;
use crate::atium::common::model::{InfoFormat, InfoOutputType, ThumbnailResponse};
use crate::atium::common::thumbnail_service::FFMPEGThumbnailService;
use crate::converter::model::{ConversionInput, ConversionRequest, ConversionResponse, get_width_height, InputSourceType, OutputResolution};


pub struct FFMPEGConversionService {
    command_manager: CommandManager
}

impl FFMPEGConversionService {
    fn extract_info(&self, file: &String) -> Result<AnalysisOutput, AtiumError> {
        let service = MediaInfoExtractorService::new()?;
        let request = InfoExtractorRequest {
            input: file.to_string(),
            format: Some(InfoFormat::Json),
            full: None,
            output_file: None,
            output_type: Some(InfoOutputType::Plain)
        };

        let response = service.get_info(request)?
            .output
            .content
            .unwrap_or(String::from(""));


        let media_info_loader = MediaInfoJsonLoader{};
        match media_info_loader.load_json_from_string(&response) {
            Ok(output) => {
                debug!("Analysis done!");
                Ok(output)
            }
            Err(err) => {
                error!("Could not analyze input file: {}", err);
                Err(err)
            }
        }
    }
    fn load_source_file(&self, source: ConversionInput) -> Result<String, &'static str> {
        match source.source_type {
            InputSourceType::Local => {
                let uuid = Uuid::new_v4().to_string();
                let mut new_path = String::from("/tmp/");
                new_path.push_str(uuid.as_str());
                new_path.push_str(".mp4");

                match fs::copy(source.file_name, new_path.clone()) {
                    Ok(_) => {
                        debug!("Successfully copied source file!");
                        Ok(new_path)
                    },
                    Err(err) => {
                        error!("Error when trying to copy input file: {}", err);
                        Err("Error when trying to copy input file")
                    }
                }
            }
        }
    }
    fn cleanup_tmp_file(&self, tmp_filepath: String) {
        match fs::remove_file(tmp_filepath) {
            Ok(_) => debug!("Temporary file removed successfully"),
            Err(err) => warn!("Temporary file not removed: {}", err)
        }
    }
    fn compute_resolution(&self, resolution: OutputResolution, current_resolution: (String,String)) -> Result<(i32,i32), AtiumError> {
        let (width, height) = get_width_height(resolution);
        let current_width = current_resolution.0.parse::<i32>()
            .map_err(|_| AtiumError::IOError("Could not convert to Integer".to_string()))?;
        let current_height = current_resolution.1.parse::<i32>()
            .map_err(|_| AtiumError::IOError("Could not convert to Integer".to_string()))?;

        let mut width = width;
        let mut height = height;
        if width > current_width {
            width = current_width;
        }
        if height > current_height {
            height = current_height;
        }

        Ok((width, height))
    }
    fn build_args(&self, resolution: OutputResolution, analysis_output: AnalysisOutput, input_file_path: String, output_file: String) -> Result<Vec<String>, AtiumError> {
        let curr_width = analysis_output.extract_field_from_track(1, &"Width".to_string())?;
        let curr_height = analysis_output.extract_field_from_track(1, &"Height".to_string())?;

        let (width, height) = self.compute_resolution(resolution, (curr_width, curr_height))?;

        debug!("Requested resolution is [{}x{}]", width, height);

        Ok(vec![
            String::from("-i"),
            input_file_path,
            String::from("-vf"),
            format!("scale={}:{}", width, height),
            output_file
        ])
    }
    fn extract_thumbnail(&self, thumbnail_request: Option<ThumbnailRequest>, video_file: String, analysis_output: AnalysisOutput) -> Option<ThumbnailResponse> {
        return match thumbnail_request {
            None => {
                debug!("Thumbnail extraction not requested");
                None
            },
            Some(req) => {
                match FFMPEGThumbnailService::new() {
                    Ok(service) => {
                        let duration = analysis_output
                            .extract_field_from_track(0, &"Duration".to_string())
                            .unwrap_or("1.0".to_string());
                        let input_file =
                        if req.input_file.is_none() {
                                Some(video_file)
                            } else {
                                req.input_file
                            };
                        let request = ThumbnailRequest {
                            timestamp: req.timestamp,
                            input_file,
                            output_file: req.output_file,
                            input_duration: Some(duration)
                        };

                        match service.extract_thumbnail(request) {
                            Ok(response) => Some(response),
                            Err(err) => {
                                error!("An error occurred when extracting thumbnail [{}]", err);
                                None
                            }
                        }
                    }
                    Err(err) => panic!("{}", err)
                }
            }
        }
    }
    /// Converts a media info and returns a [`ConversionResponse`]
    pub fn convert(&self, conversion_request: ConversionRequest) -> Result<ConversionResponse, AtiumError> {
        let input_file_path = self.load_source_file(conversion_request.input)
            .map_err(|err_msg| AtiumError::ConversionError(err_msg.to_string()))?;

        let analysis_output = self.extract_info(&input_file_path)?;

        let output_file = compute_output_file(&conversion_request.output.file, "mp4")?;
        let built_args = self.build_args(
            conversion_request.output.resolution,
            analysis_output.clone(),
            input_file_path.clone(),
            output_file.clone())
            .map_err(|err_msg| AtiumError::ConversionError(err_msg.to_string()))?;

        debug!("Converting file at path [{}]", input_file_path);

        match self.command_manager.execute_with_args(built_args.iter().map(AsRef::as_ref).collect()) {
            Ok(result) => {
                if !result.status.success() {
                    self.command_manager.print_command_output(result.stderr)?;
                    return Err(AtiumError::ConversionError("Execution of command returned ERROR".to_string()))
                }

                self.cleanup_tmp_file(input_file_path);

                debug!("Conversion done!");

                Ok(ConversionResponse {
                    output_file: output_file.clone(),
                    thumbnail_response: self.extract_thumbnail(conversion_request.output.thumbnail_request, output_file, analysis_output)
                })
            }
            Err(_) => {
                self.cleanup_tmp_file(input_file_path);
                Err(AtiumError::ConversionError("conversion command execution failed".to_string()))
            }
        }
    }
    /// Constructs a new instance of [`FFMPEGConversionService`]
    pub fn new() -> Result<Self, AtiumError> {
        let command_manager =
            CommandManager::new("ffmpeg".to_string(), vec!["-version"])?;

        Ok(Self { command_manager })
    }
}