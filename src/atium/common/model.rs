/// A Thumbnail extraction request
#[derive(Clone)]
pub struct ThumbnailRequest {
    /// A timestamp with format `hh:mm:ss`
    pub timestamp: Option<String>,
    /// The filepath from where the thumbnail will be extracted
    pub input_file: Option<String>,
    /// A filepath where the thumbnail will be saved
    pub output_file: Option<String>,
    /// Input file duration. If None, it will be computed afterwards
    pub input_duration: Option<String>
}

impl ThumbnailRequest {
    /// Create a new [`ThumbnailRequest`] which is None if timestamp and output are None
    /// otherwise is Some
    pub fn new(
        timestamp: &Option<String>,
        input_file: &Option<String>,
        output_file: &Option<String>
    ) -> Option<ThumbnailRequest> {
        if output_file.is_none() && input_file.is_none() {
            return None
        }

        Some(
            ThumbnailRequest {
                timestamp: timestamp.clone(),
                input_file: input_file.clone(),
                output_file: output_file.clone(),
                // Will be computed later
                input_duration: None
            }
        )
    }
}

pub struct ThumbnailResponse {
    pub output: String
}


/// Supported values for infos output format
#[derive(Clone)]
pub enum InfoFormat {
    Json, Html, Xml
}

pub fn parse_info_format(input: Option<String>) -> Option<InfoFormat> {
    input.map(|format| match format.to_lowercase().as_str() {
        "json" => InfoFormat::Json,
        "html" => InfoFormat::Html,
        "xml" => InfoFormat::Xml,
        _ => InfoFormat::Json
    })
}

#[derive(Clone)]
pub enum InfoOutputType {
    /// Output will be printed to Stdout
    Stdout,
    /// Output will be written to a file
    File,
    /// Output will be returned as a [`String`]
    Plain
}

pub fn parse_info_output_type(input: Option<String>) -> Option<InfoOutputType> {
    input.map(|input| match input.to_lowercase().as_str() {
        "std" => InfoOutputType::Stdout,
        "file" => InfoOutputType::File,
        _ => InfoOutputType::Stdout
    })
}

/// It represents the request used to extract infos from a video file
#[derive(Clone)]
pub struct InfoExtractorRequest {
    /// The input file path
    pub input: String,
    /// The analysis output format
    /// If None, the default will be set to [`InfoFormat::Json`]
    /// Otherwise the passed [`InfoFormat`] value will be used
    pub format: Option<InfoFormat>,
    /// The full output is requested
    /// If None, the default value will be set to `true`
    /// Otherwise the passed boolean will be used
    pub full: Option<bool>,
    /// An output file is requested
    /// If None, the output will be written to stdout
    /// Otherwise the output will be written to a file at the provided path
    pub output_file: Option<String>,
    /// If Some of [`InfoOutputType`] it contains one of Stdout, File, Plain options
    /// Default to Stdout
    pub output_type: Option<InfoOutputType>
}

/// It represents the response output of the extracted info
pub struct InfoExtractorResponseOutput {
    /// If Some it contains the path to the written file containing extracted infos
    pub file: Option<String>,
    /// If Some it contains the analysis output
    pub content: Option<String>
}

/// It represents the response of the extracted info
pub struct InfoExtractorResponse {
    /// The [`InfoExtractorResponseOutput`] containing pointers to the actual output
    pub output: InfoExtractorResponseOutput
}