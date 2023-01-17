use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::{debug, error};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::atium::common::error::AtiumError;


#[derive(Serialize, Deserialize)]
pub struct Media {
    pub track: Vec<HashMap<String,Value>>
}

#[derive(Serialize, Deserialize)]
pub struct AnalysisOutput {
    pub media: Media
}

impl AnalysisOutput {
    fn parse_string_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.to_string(),
            _ => String::from(""),
        }
    }
    /// Tries to extract a value from the media info output
    pub fn extract_field_from_track(&self, track_number: usize, field_name: &String) -> Result<String, AtiumError> {
        match self.media.track.get(track_number) {
            Some(track) => {
                track.get(field_name)
                    .map(|v| self.parse_string_value(v))
                    .ok_or(AtiumError::IOError("Could not extract field from track".to_string()))
            }
            None => Err(AtiumError::IOError("Could not extract track".to_string()))
        }
    }
}

pub struct MediaInfoJsonLoader{}
impl MediaInfoJsonLoader {
    fn deserialize(&self, data: &str) -> Result<AnalysisOutput, AtiumError> {
        let analysis_output: AnalysisOutput = serde_json::from_str(data)
            .map_err(|err| {
                error!("Error when parsing {}", err);
                AtiumError::IOError("Could not parse json".to_string())
            })?;

        Ok(analysis_output)
    }
    /// Loads JSON structure into [`AnalysisOutput`] struct starting from an input file
    pub fn load_json_from_file(&self, input_file: &String) -> Result<AnalysisOutput, AtiumError> {
        match fs::read_to_string(input_file) {
            Ok(data) => {
                debug!("File read successfully");
                self.deserialize(&data)
            }
            Err(err) => {
                error!("Error when reading JSON file: {}", err);
                Err(AtiumError::IOError(err.to_string()))
            }
        }
    }
    /// Loads JSON structure into [`AnalysisOutput`] struct starting from an input [`String`]
    pub fn load_json_from_string(&self, input: &String) -> Result<AnalysisOutput, AtiumError> {
        self.deserialize(input.as_str())
    }
}

pub fn compute_output_file(output: &String) -> String {
    let path = Path::new(output);

    if path.exists() {
        let uuid = &Uuid::new_v4().to_string()[0..7];
        let splitted =
            output
                .split('.')
                .collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        let random_n = rng.gen_range(0..10000).to_string();
        let name = splitted.get(0).unwrap_or(&uuid).to_string();
        let extension = splitted.get(1).unwrap_or(&"mp4").to_string();

        return format!("{}-{}.{}", name, random_n, extension)
    }

    output.clone()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_json_loading() {
        let json_loader = MediaInfoJsonLoader{};
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/info.json");

        let result = json_loader.load_json_from_file(&d.to_str().unwrap().to_string());

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_extract_fields() {
        let json_loader = MediaInfoJsonLoader{};
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/info.json");

        let output = json_loader.load_json_from_file(&d.to_str().unwrap().to_string()).unwrap();

        let result = output.extract_field_from_track(1, &String::from("Width"));

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap(), String::from("1920"))
    }
}