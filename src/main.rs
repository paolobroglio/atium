use clap::{Parser, Subcommand};

use atium::converter;

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
    },
    Analyze {
        input: String,
        #[arg(short, long)]
        full: Option<bool>,
        #[arg(long)]
        output_format: Option<String>,
        #[arg(long)]
        output_file: Option<String>
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
            resolution
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
                    codec: OutputCodec::H264
                }
            };

            match conversion_service.convert(request) {
                Ok(response) => {
                    println!("Converted file available at {}", response.output_file)
                }
                Err(msg) => eprintln!("An error occurred when converting {}", msg)
            }

        }
    }
}
