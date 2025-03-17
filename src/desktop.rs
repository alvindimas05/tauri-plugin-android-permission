use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<AndroidPermission<R>> {
  Ok(AndroidPermission(app.clone()))
}

/// Access to the android-permission APIs.
pub struct AndroidPermission<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AndroidPermission<R> {
    pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
        Ok(PermissionStatus::default())
    }

    pub fn request_permissions(
        &self,
        _: Option<Vec<PermissionType>>,
    ) -> crate::Result<PermissionStatus> {
        Ok(PermissionStatus::default())
    }
}
