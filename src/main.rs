use std::process::Command;
use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::Rng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input_path: String,
    output_path: String,

    #[arg(short, long, default_value_t=0)]
    thumb: i32,
    #[arg(long)]
    thumb_ts: Option<String>,
    #[arg(long)]
    thumb_out: Option<String>,

    #[arg(short,long, default_value_t=0)]
    debug: i32
}

struct ThumbnailOptions {
    timestamp: String,
    output: String
}

impl ThumbnailOptions {
    fn new(timestamp: String, output: String) -> ThumbnailOptions {
        ThumbnailOptions {
            timestamp,
            output
        }
    }
}

struct ConversionOptions {
    input_path: String,
    output_path: String,

    thumb_options: Option<ThumbnailOptions>,

    debug: bool
}

impl ConversionOptions {
    fn new(input: String, output: String, thumbnail_options: Option<ThumbnailOptions>, debug: bool) -> ConversionOptions {
        ConversionOptions {
            input_path: input,
            output_path: output,
            thumb_options: thumbnail_options,
            debug
        }
    }
    fn from_cli(cli: Cli) -> ConversionOptions {
        ConversionOptions::new(
            cli.input_path,
            cli.output_path,
            {
                if cli.thumb == 1 {
                    let thumbnail_output_path = match cli.thumb_out {
                        Some(thumb_out) => thumb_out,
                        None => String::from("thumb.jpeg")
                    };

                    let thumbnail_timestamp = match cli.thumb_ts {
                        Some(thumb_ts) => thumb_ts,
                        None => String::from("1")
                    };

                    Some(ThumbnailOptions::new(thumbnail_timestamp, thumbnail_output_path))
                } else {
                    None
                }
            },
            cli.debug == 1
        )
    }
}

fn print_process_output(output: std::process::Output, is_err: bool) {
    let mut out = output.stdout;
    if is_err {
        out = output.stderr;
    }

    String::from_utf8(out)
        .expect("could not parse std")
        .lines()
        .for_each(|x| println!("{:?}", x));
}

fn execute_ffmpeg(args: &Vec<&str>, debug: bool) -> Result<(), &'static str> {
    let mut command = Command::new("ffmpeg");
    let command_with_args = command.args(args);

    if debug {
        println!("Command arguments");
        print!("ffmpeg ");
        command_with_args.get_args()
            .for_each(|x| print!("{} ", x.to_str().unwrap()));
        println!()
    }
    let execution = command_with_args.output().expect("could not execute conversion");
    if !execution.status.success() {
        if debug {
            print_process_output(execution, true);
        }

        return Err("Command execution failed")
    }

    Ok(())
}

fn convert(options: ConversionOptions) -> Result<(), &'static str> {
    let rnd_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();

    let splitted_output = options.output_path
        .split('.')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let name = splitted_output.get(0)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "output".to_string());

    let extension = splitted_output.get(1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "mp4".to_string());

    let converted_name = format!("{}_{}.{}", name, rnd_name, extension);

    let args = vec![
        "-i",
        options.input_path.as_str(),
        converted_name.as_str()
    ];

    match execute_ffmpeg(&args, options.debug) {
        Ok(_) => {
            return match options.thumb_options {
                Some(thumb_options) => {
                    let thumb_name = format!("{}_{}.jpeg", name, rnd_name);
                    let args = vec![
                        "-i",
                        converted_name.as_str(),
                        "-ss",
                        thumb_options.timestamp.as_str(),
                        "-vframes",
                        "1",
                        thumb_name.as_str()
                    ];
                    execute_ffmpeg(&args, options.debug)
                }
                None => Ok(())
            }
        }
        Err(err) => Err(err)
    }
}


fn main() {
    let cli = Cli::parse();
    let conversion_options = ConversionOptions::from_cli(cli);

    match convert(conversion_options) {
        Ok(_) => println!("Command executed successfully"),
        Err(msg) => println!("An error occurred {}", msg)
    }
}
