use clap::{Parser, Subcommand};

use atium::converter;

use crate::atium::utility::model::{InfoExtractorEngine, InfoExtractorRequest, parse_info_format};
use crate::atium::utility::service::{InfoExtractorBuilder};

mod atium;

#[derive(Subcommand)]
enum Commands {
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
        }
    }
}
