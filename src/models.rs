use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::plugin::PermissionState;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub audio: PermissionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PermissionType {
    Audio,
    Images,
    Video,
}
