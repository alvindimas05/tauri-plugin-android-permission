use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;
pub mod models;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AndroidPermission;
#[cfg(mobile)]
use mobile::AndroidPermission;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the android-permission APIs.
pub trait AndroidPermissionExt<R: Runtime> {
  fn android_permission(&self) -> &AndroidPermission<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidPermissionExt<R> for T {
  fn android_permission(&self) -> &AndroidPermission<R> {
    self.state::<AndroidPermission<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("androidpermission")
    .invoke_handler(tauri::generate_handler![commands::request_permissions, commands::check_permissions])
    .setup(|app, api| {
      #[cfg(mobile)]
      let android_permission = mobile::init(app, api)?;
      #[cfg(desktop)]
      let android_permission = desktop::init(app, api)?;
      app.manage(android_permission);
      Ok(())
    })
    .build()
}
