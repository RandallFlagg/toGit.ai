use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

//TODO: Should all the String be changed to &str?
#[derive(Debug, Serialize)]
pub(crate) struct FileMetadata {
    pub id: usize,
    pub file_name: String,
    pub file_extension: String,
    pub file_type: String,
    pub file_status: String,
    pub size: String,
    // pub created_by: String,
    // pub created_at: String, //rfc_339
    // pub modified_by: String,
    // pub modified_at: String, //rfc_339
    // pub comments: String,
    // pub preview: String,
    // pub selected: bool,
    pub full_file_path: String,
    pub relative_file_path: String,
}

impl FileMetadata {
    fn to_(&self) -> Vec<(String, String)> {
        let json_value = serde_json::to_value(self).unwrap();
        let json_object = json_value.as_object().unwrap();

        json_object.iter().map(|(k, v)| (k.clone(), v.to_string())).collect()
    }
}
