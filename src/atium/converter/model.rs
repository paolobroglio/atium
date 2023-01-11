pub enum ConversionEngine {
    Ffmpeg
}

pub enum InputSourceType {
    Web,
    Local
}

pub struct ConversionInput {
    source_type: InputSourceType,
    file_name: String
}

pub enum OutputResolution {
    Hd, FullHd, FullHd2k, UltraHd, FullUltraHd
}

pub enum OutputCodec {
    H264, Vp9
}

pub struct ConversionOutput {
    file: String,
    resolution: Option<OutputResolution>,
    codec: Option<OutputCodec>
}

pub struct ConversionRequest {
    input: ConversionInput,
    output: ConversionOutput
}

pub struct ConversionResponse {
    output_file: String
}