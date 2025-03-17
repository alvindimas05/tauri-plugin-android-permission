use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::AndroidPermissionExt;
use crate::Result;

#[command]
#[specta::specta]
pub(crate) async fn check_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.android_permission().check_permissions()
}

#[command]
#[specta::specta]
pub(crate) async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    permissions: Option<Vec<PermissionType>>,
) -> Result<PermissionStatus> {
    app.android_permission().request_permissions(permissions)
}
