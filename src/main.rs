use clap::{Parser, Subcommand};

use atium::converter;
use crate::atium::common::error::AtiumError;
use crate::atium::common::model::{ThumbnailRequest, ThumbnailResponse};
use crate::atium::common::service::ThumbnailServiceBuilder;

use crate::atium::utility::model::{InfoExtractorEngine, InfoExtractorRequest, parse_info_format};
use crate::atium::utility::service::InfoExtractorBuilder;
use crate::converter::model::{ConversionEngine, ConversionInput, ConversionOutput, ConversionRequest, InputSourceType, OutputCodec, parse_resolution};
use crate::converter::service::ConversionServiceBuilder;

mod atium;

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        source_type: Option<String>,
        #[arg(short, long)]
        output: String,
        #[arg(short, long)]
        resolution: String,
        #[arg(long)]
        thumb_ts: Option<String>,
        #[arg(long)]
        thumb_source: Option<String>,
        #[arg(long)]
        thumb_out: Option<String>
    },
    Analyze {
        input: String,
        #[arg(short, long)]
        full: Option<bool>,
        #[arg(long)]
        output_format: Option<String>,
        #[arg(long)]
        output_file: Option<String>
    },
    Tool {
        #[arg(long)]
        thumb_ts: Option<String>,
        #[arg(long)]
        thumb_source: Option<String>,
        #[arg(long)]
        thumb_out: Option<String>
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
                        println!("Output written to {}", response.output.file.unwrap())
                    }
                    println!("Info extracted successfully")
                }
                Err(err) => eprintln!("An error occurred when extracting info {}", err)
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
                    println!("Converted file available at [{}]", response.output_file)
                }
                Err(msg) => eprintln!("An error occurred when converting {}", msg)
            }
        },
        Commands::Tool {
            thumb_ts,
            thumb_source,
            thumb_out
        } => {
            let request = ThumbnailRequest::new(
                thumb_ts,
                thumb_source,
                thumb_out
            );
            let engine = ConversionEngine::Ffmpeg;
            let service = ThumbnailServiceBuilder::new(engine)
                .expect("Could not build service!");

            if request.is_none() {
                eprintln!("You didn't specify all the required options!")
            } else {
                match service.extract_thumbnail(request.unwrap()) {
                    Ok(_) => println!("Thumbnail extracted successfully"),
                    Err(err) => eprintln!("An error occurred when extracting thumbnail: {}", err)
                }
            }

        }
    }
}
