use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::plugin::PermissionState;

// Note: Don't forget to add your permission name here if you add it manually.

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub read_media_audio: PermissionState,
    pub read_media_images: PermissionState,
    pub read_media_video: PermissionState,
    pub read_external_storage: PermissionState,
    pub write_external_storage: PermissionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PermissionType {
    ReadMediaAudio,
    ReadMediaImages,
    ReadMediaVideo,
    ReadExternalStorage,
    WriteExternalStorage,
}
