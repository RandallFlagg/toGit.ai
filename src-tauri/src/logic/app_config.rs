use serde::{Deserialize, Serialize};
//TODO: Should this be here or in the js?
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct AppConfig {
    pub background_color: String,
    pub local_branch_color: String,
    pub active_branch_color: String,
    pub remote_branch_color: String,
    pub remote_tag_color: String,
    pub local_tag_color: String,
    pub type_order: Vec<String>, // Type priority order
}

impl AppConfig {
    fn default() -> Self {
        Self {
            background_color: "#FFFFFF".to_string(),
            local_branch_color: "#00FF00".to_string(),
            active_branch_color: "#FF0000".to_string(),
            remote_branch_color: "#F5F5DC".to_string(),
            remote_tag_color: "#FFFF00".to_string(),
            local_tag_color: "#FFFFE0".to_string(),
            type_order: vec![
                "LocalTag".to_string(),
                "RemoteTag".to_string(),
                "LocalBranch".to_string(),
                "RemoteBranch".to_string(),
                "ActiveBranch".to_string(),
            ],
        }
    }
}
