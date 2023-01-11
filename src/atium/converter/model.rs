/// The engine used for conversion
pub enum ConversionEngine {
    Ffmpeg
}

/// The input file source type
pub enum InputSourceType {
    Web,
    Local
}

/// The proper input file name and its source type
pub struct ConversionInput {
    pub source_type: InputSourceType,
    pub file_name: String
}

/// Output resolution options:
/// * Hd          -> 720p  - 1280x720
/// * FullHd      -> 1080p - 1920x1080
/// * FullHd2k    -> 1080p - 2048x1080
/// * UltraHd     -> 4k    - 3840x2160
/// * FullUltraHd -> 8k    - 7680x4320
pub enum OutputResolution {
    Hd, FullHd, FullHd2k, UltraHd, FullUltraHd
}

/// Output codec options
pub enum OutputCodec {
    H264, Vp9
}

/// Conversion output options
pub struct ConversionOutput {
    pub file: String,
    pub resolution: Option<OutputResolution>,
    pub codec: Option<OutputCodec>
}

/// Conversion request containing options for input and output
pub struct ConversionRequest {
    pub input: ConversionInput,
    pub output: ConversionOutput
}

/// Conversion response containing the output filepath
pub struct ConversionResponse {
    pub output_file: String
}