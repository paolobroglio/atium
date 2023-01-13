/// A Thumbnail extraction request
pub struct ThumbnailRequest {
    /// A timestamp with format `hh:mm:ss`
    pub timestamp: Option<String>,
    /// The filepath from where the thumbnail will be extracted
    pub input_file: Option<String>,
    /// A filepath where the thumbnail will be saved
    pub output_file: Option<String>,
}

impl ThumbnailRequest {
    /// Create a new [`ThumbnailRequest`] which is None if timestamp and output are None
    /// otherwise is Some
    pub fn new(timestamp: &Option<String>, input_file: &Option<String>, output_file: &Option<String>) -> Option<ThumbnailRequest> {
        if timestamp.is_none() && output_file.is_none() && input_file.is_none() {
            return None
        }

        Some(
            ThumbnailRequest {
                timestamp: timestamp.clone(),
                input_file: input_file.clone(),
                output_file: output_file.clone()
            }
        )
    }
}

pub struct ThumbnailResponse {
    pub output: String
}