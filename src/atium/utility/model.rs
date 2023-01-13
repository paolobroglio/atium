/// Supported values for infos output format
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

pub enum InfoExtractorEngine {
    MediaInfo
}

/// It represents the request used to extract infos from a video file
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
    pub output_file: Option<String>
}

/// It represents the response output of the extracted info
pub struct InfoExtractorResponseOutput {
    /// If Some it contains the path to the written file containing extracted infos
    pub file: Option<String>
}

/// It represents the response of the extracted info
pub struct InfoExtractorResponse {
    /// The [`InfoExtractorResponseOutput`] containing pointers to the actual output
    pub output: InfoExtractorResponseOutput
}