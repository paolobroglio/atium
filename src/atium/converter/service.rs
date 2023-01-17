use std::fs;
use std::path::Path;

use log::{debug, error, warn};
use rand::Rng;
use uuid::Uuid;

use crate::{InfoExtractorBuilder, InfoExtractorEngine, InfoExtractorRequest, ThumbnailRequest};
use crate::atium::common::command_manager::CommandManager;
use crate::atium::common::error::AtiumError;
use crate::atium::common::model::ThumbnailResponse;
use crate::atium::common::service::ThumbnailServiceBuilder;
use crate::atium::utility::helper::{AnalysisOutput, MediaInfoJsonLoader};
use crate::atium::utility::model::{InfoFormat, InfoOutputType};
use crate::converter::model::{ConversionEngine, ConversionInput, ConversionRequest, ConversionResponse, get_width_height, InputSourceType, OutputResolution};

/// Conversion service that holds the logic for converting a video content
pub trait ConversionService {
    /// Converts a given input video file by using the previously loaded engine.
    ///
    /// # Arguments
    ///
    /// * `conversion_request` - An instance of [`ConversionRequest`] struct
    ///
    /// # Examples
    /// ```
    /// let selected_engine = ConversionEngine::Ffmpeg;
    /// let conversion_service =
    ///     ConversionServiceBuilder::new(selected_engine)
    ///     .expect("could not load service");
    /// let request = ConversionRequest{
    ///     input: ConversionInput {
    ///         source_type: InputSourceType::Local,
    ///         file_name:  input.clone()
    ///     },
    ///     output: ConversionOutput {
    ///         file: output.clone(),
    ///         resolution: None,
    ///         codec: None
    ///     }
    /// };
    /// let result = conversion_service.convert(request);
    /// ```
    fn convert(&self, conversion_request: ConversionRequest) -> Result<ConversionResponse, AtiumError>;
}

/// FFMPEG implementation of [`ConversionService`] behavior
pub struct FFMPEGConversionService {
    command_manager: CommandManager
}

impl FFMPEGConversionService {
    fn extract_info(&self, file: &String) -> Result<AnalysisOutput, AtiumError> {
        let engine = InfoExtractorEngine::MediaInfo;
        let service = InfoExtractorBuilder::new(engine)?;
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
    fn compute_output_file(&self, output: &String) -> String {
        let path = Path::new(output);

            if path.exists() {
                let uuid = &Uuid::new_v4().to_string()[0..7];
                let splitted =
                    output
                        .split('.')
                        .collect::<Vec<_>>();
                let mut rng = rand::thread_rng();
                let random_n = rng.gen_range(0..10000).to_string();
                let name = splitted.get(0).unwrap_or(&uuid).to_string();
                let extension = splitted.get(1).unwrap_or(&"mp4").to_string();

                return format!("{}-{}.{}", name, random_n, extension)
            }

            output.clone()
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
        fn extract_thumbnail(&self, thumbnail_request: Option<ThumbnailRequest>, video_file: String) -> Option<ThumbnailResponse> {
            return match thumbnail_request {
                None => {
                    debug!("Thumbnail extraction not requested");
                    None
                },
                Some(req) => {
                    let thumb_engine = ConversionEngine::Ffmpeg;
                    let service = ThumbnailServiceBuilder::new(thumb_engine)
                        .expect("Could not build service!");
                    let input_file =
                        if req.input_file.is_none() {
                            Some(video_file)
                        } else {
                            req.input_file
                        };
                    let request = ThumbnailRequest {
                        timestamp: req.timestamp,
                        input_file,
                        output_file: req.output_file
                    };
                    match service.extract_thumbnail(request) {
                        Ok(response) => Some(response),
                        Err(err) => {
                            error!("An error occurred when extracting thumbnail [{}]", err);
                            None
                        }
                    }
                }
            }
        }
}

impl ConversionService for FFMPEGConversionService{
    fn convert(&self, conversion_request: ConversionRequest) -> Result<ConversionResponse, AtiumError> {
        let input_file_path = self.load_source_file(conversion_request.input)
            .map_err(|err_msg| AtiumError::ConversionError(err_msg.to_string()))
            .expect("could not load source file");

        let analysis_output = self.extract_info(&input_file_path)?;

        let output_file = self.compute_output_file(&conversion_request.output.file);
        let built_args = self.build_args(
            conversion_request.output.resolution,
            analysis_output,
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
                    thumbnail_response: self.extract_thumbnail(conversion_request.output.thumbnail_request, output_file)
                })
            }
            Err(_) => {
                self.cleanup_tmp_file(input_file_path);
                Err(AtiumError::ConversionError("conversion command execution failed".to_string()))
            }
        }
    }
}

/// This struct lets you build a new [`ConversionService`] based on given engine
pub struct ConversionServiceBuilder {}

impl ConversionServiceBuilder {
    /// Creates a new instance of [`ConversionService`] with the requested loaded engine.
    /// Current supported engines are:
    /// * `ffmpeg`
    ///
    /// # Arguments
    ///
    /// * `engine` - Any value of [`ConversionEngine`] enum
    ///
    /// # Examples
    /// ```
    /// let selected_engine = ConversionEngine::Ffmpeg;
    /// let conversion_service = ConversionServiceBuilder::new(selected_engine).expect("error!");
    /// ```
    pub fn new(engine: ConversionEngine) -> Result<Box<dyn ConversionService>, AtiumError> {
        return match engine {
            ConversionEngine::Ffmpeg => {
                debug!("Creating a new FFMPEG service");
                let command_manager =
                    CommandManager::new("ffmpeg".to_string(), vec!["-version"])
                        .expect("could not load command!");
                debug!("FFMPEG service created!");
                Ok(Box::new(FFMPEGConversionService {
                    command_manager
                }))
            }
        }
    }
}