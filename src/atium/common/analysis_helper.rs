use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::{debug, error};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::atium::common::error::AtiumError;


#[derive(Clone, Serialize, Deserialize)]
pub struct Media {
    pub track: Vec<HashMap<String,Value>>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AnalysisOutput {
    pub media: Media
}

impl AnalysisOutput {
    fn parse_string_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => {
                debug!("Extracted field value: [{}]", s);
                s.to_string()
            },
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

fn get_file_name_from_path(path: &Path) -> Result<String, AtiumError> {
    let name_os = path.file_stem()
        .ok_or(AtiumError::IOError("Could not parse filename".to_string()))?
        .to_os_string();

    let uuid = Uuid::new_v4().to_string();

    Ok(name_os.to_str()
        .unwrap_or(uuid.as_str())
        .to_string())
}

fn get_extension_from_path(path: &Path, default_extension: &str) -> Result<String, AtiumError> {
    let name_os = path.extension()
        .ok_or(AtiumError::IOError("Could not parse extension".to_string()))?
        .to_os_string();

    Ok(name_os.to_str()
        .unwrap_or(default_extension)
        .to_string())
}

pub fn compute_output_file(output: &String, default_extension: &str) -> Result<String, AtiumError> {
    let path = Path::new(output);
    if path.exists() {
        let mut rng = rand::thread_rng();
        let random_n = rng.gen_range(0..10000).to_string();
        let name = get_file_name_from_path(path)?;
        let extension = get_extension_from_path(path, default_extension)?;

        return Ok(format!("{}-{}.{}", name, random_n, extension))
    }

    Ok(output.clone())
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

        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_fields() {
        let json_loader = MediaInfoJsonLoader{};
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/info.json");

        let output = json_loader.load_json_from_file(&d.to_str().unwrap().to_string()).unwrap();

        let result = output.extract_field_from_track(1, &String::from("Width"));

        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), String::from("1920"))
    }

    #[test]
    fn test_make_output_path() {
        let result = compute_output_file(&String::from("/Users/user.name/path/to/video.mp4"), "mp4");
        println!("{}", result.ok().unwrap());

        // [WARNING] needs creating a tmp file
        fs::write("/tmp/example.mp4", b"");
        let result = compute_output_file(&String::from("/tmp/example.mp4"), "mp4");
        println!("{}", result.ok().unwrap());
    }

    #[test]
    fn test_get_name_from_path() {
        let result = get_file_name_from_path(&Path::new("/tmp/example.mp4"));

        assert_eq!(result.ok().unwrap(), String::from("example"));
    }

    #[test]
    fn test_get_extension_from_path() {
        let result = get_extension_from_path(&Path::new("/tmp/example.mp4"), "mp4");
        assert_eq!(result.ok().unwrap(), String::from("mp4"));

        let point_in_path = get_extension_from_path(&Path::new("/Users/user.name/dir/example.mp4"), "mp4");
        assert_eq!(point_in_path.ok().unwrap(), String::from("mp4"));
    }
}