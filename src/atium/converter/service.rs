use std::fs;
use uuid::Uuid;
use crate::atium::common::command_manager::CommandManager;
use crate::converter::model::{ConversionEngine, ConversionInput, ConversionRequest, ConversionResponse, InputSourceType};

/// Conversion service that holds the logic for converting a video content
pub trait ConversionService {
    /// Converts a given input video file by using the previously loaded engine.
    ///
    /// # Arguments
    ///
    /// * `request` - An instance of [`ConversionRequest`] struct
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
    fn convert(&self, conversion_request: ConversionRequest) -> Result<ConversionResponse, &'static str>;
}

/// FFMPEG implementation of [`ConversionService`] behavior
pub struct FFMPEGConversionService {
    command_manager: CommandManager
}

impl FFMPEGConversionService {
    fn load_source_file(&self, source: ConversionInput) -> Result<String, &'static str> {
        match source.source_type {
            InputSourceType::Web => {
                Err("Web source type is currently not supported!")
            }
            InputSourceType::Local => {
                let uuid = Uuid::new_v4().to_string();
                let mut new_path = String::from("/tmp/");
                new_path.push_str(uuid.as_str());
                new_path.push_str(".mp4");

                match fs::copy(source.file_name, new_path.clone()) {
                    Ok(_) => Ok(new_path),
                    Err(_) => Err("Error when trying to copy input file")
                }
            }
        }
    }
    fn cleanup_tmp_file(&self, tmp_filepath: String) {
        match fs::remove_file(tmp_filepath) {
            Ok(_) => println!("Temporary file removed successfully"),
            Err(_) => eprintln!("Temporary file not removed!")
        }
    }
}

impl ConversionService for FFMPEGConversionService{
    fn convert(&self, conversion_request: ConversionRequest) -> Result<ConversionResponse, &'static str> {
        let input_file_path = self.load_source_file(conversion_request.input)
            .expect("could not load source file");
        let output_file = conversion_request.output.file.as_str();

        println!("Converting file at path [{}]", input_file_path);
        println!("Output will be available at path [{}]", output_file);

        match self.command_manager.execute_with_args(vec![
            "-i",
            input_file_path.as_str(),
            conversion_request.output.file.as_str()
        ]) {
            Ok(result) => {
                if !result.status.success() {
                    self.command_manager.print_command_output(result.stderr)?;
                    return Err("conversion failed!")
                }
                self.cleanup_tmp_file(input_file_path);
                Ok(ConversionResponse { output_file: output_file.to_string() })
            }
            Err(_) => {
                self.cleanup_tmp_file(input_file_path);
                Err("conversion command execution failed")
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
    pub fn new(engine: ConversionEngine) -> Result<Box<dyn ConversionService>, &'static str> {
        return match engine {
            ConversionEngine::Ffmpeg => {
                let command_manager =
                    CommandManager::new("ffmpeg".to_string(), vec!["-version"])
                        .expect("could not load command!");

                Ok(Box::new(FFMPEGConversionService {
                    command_manager
                }))
            }
        }
    }
}