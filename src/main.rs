//! # Atium
//!
//! Atium is a simple video conversion tool that lets you obtain a converted video by
//! specifying some parameters like resolution, codec, and container.
//!
//! This is available as an importable library but its main usage occurs through a CLI,
//! powered by `Clap`.
//!
//! ## API
//! ### Conversion
//!
//! Basic usage for conversion API is simple as it follows:
//! #### Engine selection
//! First you need to select an **engine**. This is a straightforward way to identify the underlying
//! tool used for doing the actual conversion.
//! ```
//! let selected_engine = ConversionEngine::Ffmpeg;
//! ```
//! Then you must obtain a new instance of the [ConversionService](crate::converter::service::ConversionService) trait that will contain the specific
//! implementation according to the selected tool.
//! ```
//! let conversion_service = ConversionServiceBuilder::new(selected_engine).expect("could not load service");
//! ```
//! The following step is to create a [ConversionRequest](crate::converter::model::ConversionRequest) that contains the required options in order to
//! tune the conversion output.
//! Please take a look at the structure documentation to understand each field's meaning.
//! ```
//! let request = ConversionRequest{
//!     input: ConversionInput {
//!         source_type: InputSourceType::Local,
//!         file_name:  String::from("/path/to/input.mp4")
//!     },
//!     output: ConversionOutput {
//!         file: String::from("/path/to/output.mp4"),
//!         resolution: OutputResolution::Hd,
//!         codec: OutputCodec::H264,
//!         thumbnail_request: ThumbnailRequest::new(
//!               String::from("00:00:01.000"),
//!               // this could be blank since it will use the converted video
//!               // as a source for thumbnail extraction
//!               String::from("/path/to/input.mp4"),
//!               String::from("/path/to/thumb.jpg")
//!         )
//!     }
//!  };
//! ```
//! Now you can perform the actual conversion! Use the previously created `conversion_service`
//! and call the `convert` method.
//! ```
//! let result: Result<ConversionResponse, AtiumError> = conversion_service.convert(request);
//! ```
//! The output is a `Result<ConversionResponse, AtiumError>`, so now is up to you.
//! This contains a [ConversionResponse](crate::converter::model::ConversionResponse) that holds the output path of the converted video!
//!
//! ## Command Line Interface
//!
//! After installing `atium` by entering `atium --help` the following helper shows up:
//! ```text
//! Usage: atium <COMMAND>
//!
//! Commands:
//!   convert    Conversion tool for video media
//!   analyze    Analyze media to extract useful infos
//!   thumbnail  Thumbnail extraction tool
//!   help       Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help     Print help information
//!   -V, --version  Print version information
//! ```
//! Here you can access all the tools offered by `atium`.
//!
//! ### Conversion
//!
//! ```
//! Conversion tool for video media
//!
//! Usage: atium convert [OPTIONS] --input <INPUT> --output <OUTPUT> --resolution <RESOLUTION>
//!
//! Options:
//!   -i, --input <INPUT>                Input file to convert
//!   -s, --source-type <SOURCE_TYPE>    Type of source to convert
//!   -o, --output <OUTPUT>              Output path for the converted file
//!   -r, --resolution <RESOLUTION>      Requested output resolution
//!       --thumb-ts <THUMB_TS>          Timestamp requested for thumbnail extraction
//!       --thumb-source <THUMB_SOURCE>  Source from where to extract the thumbnail
//!       --thumb-out <THUMB_OUT>        Output path for the extracted thumbnail
//!   -h, --help                         Print help information
//!   -V, --version                      Print version information
//! ```
//!
//! ### Analyze
//!
//! ```
//! Analyze media to extract useful infos
//!
//! Usage: atium analyze [OPTIONS] --input <INPUT>
//!
//! Options:
//!   -i, --input <INPUT>                  Input path of the file that will be analyzed
//!   -f, --full <FULL>                    Whether you want the full analysis or not [possible values: true, false]
//!       --output-format <OUTPUT_FORMAT>  Output format of the analysis tool
//!       --output-file <OUTPUT_FILE>      Output file containing analysis result
//!   -h, --help                           Print help information
//!   -V, --version                        Print version information
//! ```
//!
//! ### Thumbnail
//!
//! ```
//! Thumbnail extraction tool
//!
//! Usage: atium thumbnail [OPTIONS]
//!
//! Options:
//!   -t, --timestamp <TIMESTAMP>      The timestamp of the video for thumbnail extraction
//!   -s, --source-path <SOURCE_PATH>  The source video for thumbnail extraction
//!   -o, --output-path <OUTPUT_PATH>  Where to put the extracted thumbnail
//!   -h, --help                       Print help information
//!   -V, --version                    Print version information
//! ```


use clap::{Parser, Subcommand};
use log::{error, info};

use atium::converter;
use crate::atium::common::model::{ThumbnailRequest};
use crate::atium::common::service::ThumbnailServiceBuilder;

use crate::atium::utility::model::{InfoExtractorEngine, InfoExtractorRequest, parse_info_format};
use crate::atium::utility::service::InfoExtractorBuilder;
use crate::converter::model::{ConversionEngine, ConversionInput, ConversionOutput, ConversionRequest, InputSourceType, OutputCodec, parse_resolution};
use crate::converter::service::ConversionServiceBuilder;

mod atium;

#[derive(Subcommand)]
enum Commands {
    /// Conversion tool for video media
    Convert {
        /// Input file to convert
        #[arg(short, long)]
        input: String,
        /// Type of source to convert
        #[arg(short, long)]
        source_type: Option<String>,
        /// Output path for the converted file
        #[arg(short, long)]
        output: String,
        /// Requested output resolution
        #[arg(short, long)]
        resolution: String,
        /// Timestamp requested for thumbnail extraction
        #[arg(long)]
        thumb_ts: Option<String>,
        /// Source from where to extract the thumbnail
        #[arg(long)]
        thumb_source: Option<String>,
        /// Output path for the extracted thumbnail
        #[arg(long)]
        thumb_out: Option<String>
    },
    /// Analyze media to extract useful infos
    Analyze {
        /// Input path of the file that will be analyzed
        #[arg(short, long)]
        input: String,
        /// Whether you want the full analysis or not
        #[arg(short, long)]
        full: Option<bool>,
        /// Output format of the analysis tool
        #[arg(long)]
        output_format: Option<String>,
        /// Output file containing analysis result
        #[arg(long)]
        output_file: Option<String>
    },
    /// Thumbnail extraction tool
    Thumbnail {
        /// The timestamp of the video for thumbnail extraction
        #[arg(short, long)]
        timestamp: Option<String>,
        /// The source video for thumbnail extraction
        #[arg(short, long)]
        source_path: Option<String>,
        /// Where to put the extracted thumbnail
        #[arg(short, long)]
        output_path: Option<String>
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
     #[command(subcommand)]
     command: Commands
}

fn main() {

    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze {
            input, output_format, full, output_file
        } => {
            let selected_engine = InfoExtractorEngine::MediaInfo;
            let info_extractor_service =
                InfoExtractorBuilder::new(selected_engine)
                    .expect("could not load service");
            let request = InfoExtractorRequest {
                input: input.to_string(),
                format: parse_info_format(output_format.clone()),
                full: *full,
                output_file: output_file.clone()
            };

            match info_extractor_service.get_info(request) {
                Ok(response) => {
                    if response.output.file.is_some() {
                        info!("Output written to {}", response.output.file.unwrap())
                    }
                    info!("Info extracted successfully")
                }
                Err(err) => error!("An error occurred when extracting info {}", err)
            }
        },
        Commands::Convert {
            input,
            source_type:_,
            output,
            resolution,
            thumb_ts,
            thumb_source,
            thumb_out
        } => {
            let selected_engine = ConversionEngine::Ffmpeg;
            let conversion_service =
                ConversionServiceBuilder::new(selected_engine)
                    .expect("could not load service");
            let request = ConversionRequest{
                input: ConversionInput {
                    source_type: InputSourceType::Local,
                    file_name:  input.clone()
                },
                output: ConversionOutput {
                    file: output.clone(),
                    resolution: parse_resolution(resolution),
                    codec: OutputCodec::H264,
                    thumbnail_request: ThumbnailRequest::new(
                        thumb_ts,
                        thumb_source,
                        thumb_out
                    )
                }
            };

            match conversion_service.convert(request) {
                Ok(response) => {
                    info!("Converted file available at [{}]", response.output_file)
                }
                Err(msg) => error!("An error occurred when converting {}", msg)
            }
        },
        Commands::Thumbnail {
            timestamp,
            source_path,
            output_path
        } => {
            let request = ThumbnailRequest::new(
                timestamp,
                source_path,
                output_path
            );
            let engine = ConversionEngine::Ffmpeg;
            let service = ThumbnailServiceBuilder::new(engine)
                .expect("Could not build service!");

            if request.is_none() {
                error!("You didn't specify all the required options!")
            } else {
                match service.extract_thumbnail(request.unwrap()) {
                    Ok(_) => info!("Thumbnail extracted successfully"),
                    Err(err) => error!("An error occurred when extracting thumbnail: {}", err)
                }
            }

        }
    }
}
