use std::path::Path;
use std::process::Output;
use rand::Rng;
use uuid::Uuid;
use crate::atium::common::command_manager::CommandManager;
use crate::atium::common::error::AtiumError;
use crate::atium::common::model::{ThumbnailRequest, ThumbnailResponse};
use crate::ConversionEngine;

/// This struct lets you build a new [`ThumbnailService`] based on given engine
pub struct ThumbnailServiceBuilder {}

impl ThumbnailServiceBuilder {
    /// Creates a new instance of [`ThumbnailService`] with the requested loaded engine.
    /// Current supported engines are:
    /// * `ffmpeg`
    ///
    /// # Arguments
    ///
    /// * `engine` - Any value of [`ThumbnailService`] enum
    ///
    /// # Examples
    /// ```
    /// let selected_engine = ConversionEngine::Ffmpeg;
    /// let thumbnail_service = ThumbnailServiceBuilder::new(selected_engine).expect("error!");
    /// ```
    pub fn new(engine: ConversionEngine) -> Result<Box<dyn ThumbnailService>, AtiumError> {
        return match engine {
            ConversionEngine::Ffmpeg => {
                let command_manager =
                    CommandManager::new("ffmpeg".to_string(), vec!["-version"])
                        .expect("could not load command!");

                Ok(Box::new(FFMPEGThumbnailService {
                    command_manager
                }))
            }
        }
    }
}


/// Thumbnail service that holds the logic for extracting a thumbnail from a video
pub trait ThumbnailService {
    /// Extracts a thumbnail from a video
    ///
    /// # Arguments
    ///
    /// * `thumbnail_request` - An instance of [`ThumbnailRequest`] struct
    ///
    /// # Examples
    /// ```
    /// ```
    fn extract_thumbnail(&self, thumbnail_request: ThumbnailRequest) -> Result<ThumbnailResponse, AtiumError>;
}

/// FFMPEG implementation of [`ThumbnailService`] behavior
pub struct FFMPEGThumbnailService {
    command_manager: CommandManager
}

impl FFMPEGThumbnailService{
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
}

impl ThumbnailService for FFMPEGThumbnailService {
    fn extract_thumbnail(&self, thumbnail_request: ThumbnailRequest) -> Result<ThumbnailResponse, AtiumError> {
        let input_file = thumbnail_request.input_file.expect("INPUT_FILE Cannot be empty");

        let mut output_file = thumbnail_request.output_file
            .unwrap_or_else(|| self.build_output_from_input_path(input_file.clone()));
        output_file = self.compute_output_file(&output_file);

        let timestamp = thumbnail_request.timestamp
            .unwrap_or_else(|| String::from("00:00:01.000"));

        let args = self.build_args(input_file, output_file.clone(), timestamp);

        match self.command_manager.execute_with_args(args.iter().map(AsRef::as_ref).collect()) {
            Ok(result) => {
                if !result.status.success() {
                    self.command_manager.print_command_output(result.stderr)?;
                    return Err(AtiumError::ConversionError("Execution of command returned ERROR".to_string()))
                }
                println!("Thumbnail extracted at path [{}]", output_file);
                Ok(ThumbnailResponse{ output: output_file })
            }
            Err(err) => Err(err)
        }
    }
}