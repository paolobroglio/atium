use crate::atium::common::model::{ThumbnailRequest, ThumbnailResponse};

/// The engine used for conversion
pub enum ConversionEngine {
    Ffmpeg
}

/// The input file source type
pub enum InputSourceType {
    Local
}

/// The proper input file name and its source type
pub struct ConversionInput {
    pub source_type: InputSourceType,
    pub file_name: String
}

/// Output resolution options:
/// * Sd          -> 480p  - 640x480
/// * Hd          -> 720p  - 1280x720
/// * FullHd      -> 1080p - 1920x1080
/// * FullHd2k    -> 1080p - 2048x1080
/// * UltraHd     -> 4k    - 3840x2160
/// * FullUltraHd -> 8k    - 7680x4320
pub enum OutputResolution {
    Sd, Hd, FullHd, FullHd2k, UltraHd, FullUltraHd
}

/// Returns a value of [`OutputResolution`] based on input:
/// Valid inputs are:
/// * sd   -> SD
/// * hd   -> HD
/// * fhd  -> FULL-HD
/// * 2k   -> FULL-HD-2K
/// * uhd  -> ULTRA-HD
/// * 8k   -> 8K
pub fn parse_resolution(resolution_string: &String) -> OutputResolution {
    match resolution_string.to_lowercase().as_str() {
        "sd" => OutputResolution::Sd,
        "hd" => OutputResolution::Hd,
        "fhd" => OutputResolution::FullHd,
        "2k" => OutputResolution::FullHd2k,
        "uhd" => OutputResolution::UltraHd,
        "8k" => OutputResolution::FullUltraHd,
        _ => OutputResolution::Hd
    }
}

/// Output resolution conversion to (width,height) tuple:
/// * Sd          -> 480p  - 640x480
/// * Hd          -> 720p  - 1280x720
/// * FullHd      -> 1080p - 1920x1080
/// * FullHd2k    -> 1080p - 2048x1080
/// * UltraHd     -> 4k    - 3840x2160
/// * FullUltraHd -> 8k    - 7680x4320
pub fn get_width_height(resolution: OutputResolution) -> (i16,i16) {
    match resolution {
        OutputResolution::Sd => (640, 480),
        OutputResolution::Hd => (1280,720),
        OutputResolution::FullHd => (1920,1080),
        OutputResolution::FullHd2k => (2048,1080),
        OutputResolution::UltraHd => (3840,2160),
        OutputResolution::FullUltraHd => (7680,4320),
    }
}

/// Output codec options
pub enum OutputCodec {
    H264
}

/// Conversion output options
pub struct ConversionOutput {
    pub file: String,
    pub resolution: OutputResolution,
    pub codec: OutputCodec,
    pub thumbnail_request: Option<ThumbnailRequest>
}

/// Conversion request containing options for input and output
pub struct ConversionRequest {
    pub input: ConversionInput,
    pub output: ConversionOutput
}

/// Conversion response containing the output filepath
pub struct ConversionResponse {
    pub output_file: String,
    pub thumbnail_response: Option<ThumbnailResponse>
}