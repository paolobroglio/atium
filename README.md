![status](https://github.com/paolobroglio/atium/actions/workflows/rust.yml/badge.svg)

# Atium

**atium** is a tool that lets you convert videos and exposes other useful tools.

# Usage

```
Usage: atium <COMMAND>

Commands:
  convert    Conversion tool for video media
  analyze    Analyze media to extract useful infos
  thumbnail  Thumbnail extraction tool
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

```

## Commands
### Conversion Tool

A simple video conversion tool
```
Conversion tool for video media

Usage: atium convert [OPTIONS] --input <INPUT> --output <OUTPUT> --resolution <RESOLUTION>

Options:
  -i, --input <INPUT>                Input file to convert
  -s, --source-type <SOURCE_TYPE>    Type of source to convert
  -o, --output <OUTPUT>              Output path for the converted file
  -r, --resolution <RESOLUTION>      Requested output resolution
      --thumb-ts <THUMB_TS>          Timestamp requested for thumbnail extraction
      --thumb-source <THUMB_SOURCE>  Source from where to extract the thumbnail
      --thumb-out <THUMB_OUT>        Output path for the extracted thumbnail
  -h, --help                         Print help information
  -V, --version                      Print version information
```

Supported resolution values are:
* `sd`
* `hd`
* `fullhd`
* `2k`
* `ultrahd`
* `8k`

## Analyze Tool
A simple analysis tool that lets you extract useful infos about media contents.

```
Analyze media to extract useful infos

Usage: atium analyze [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>                  Input path of the file that will be analyzed
  -f, --full <FULL>                    Whether you want the full analysis or not [possible values: true, false]
      --output-format <OUTPUT_FORMAT>  Output format of the analysis tool
      --output-file <OUTPUT_FILE>      Output file containing analysis result
  -h, --help                           Print help information
  -V, --version                        Print version information
```

## Thumbnail Tool
A simple thumbnail extraction tool that lets you creating a thumbnail by extracting a frame from a given video.

```
Thumbnail extraction tool

Usage: atium thumbnail [OPTIONS]

Options:
  -t, --timestamp <TIMESTAMP>      The timestamp of the video for thumbnail extraction
  -s, --source-path <SOURCE_PATH>  The source video for thumbnail extraction
  -o, --output-path <OUTPUT_PATH>  Where to put the extracted thumbnail
  -h, --help                       Print help information
  -V, --version                    Print version information
```