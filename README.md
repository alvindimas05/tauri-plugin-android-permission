# Tauri Plugin Android Permission
Tauri Plugin project template for permission request on Android.

# How To Use
- Go to `src-tauri` and clone this project
```
cd src-tauri
git clone https://github.com/alvindimas05/tauri-plugin-android-permission
```
## Installation
- Add this code at your `Cargo.toml`
```
tauri-plugin-android-permission = { path = "./tauri-plugin-android-permission" }
```

## Template
- Go to `tauri-plugin-android-permission/android/src/main/java/AndroidPermissionPlugin.kt` and uncomment the permissions code that you needed.

## Manual
- You can add manually by copy paste the code. You can see all of android permissions here. 
https://developer.android.com/reference/android/Manifest.permission
- Add your permissions at the `src/models.rs`

## Code Example
```
#[tauri::command]
pub fn request_read_audio_permission(app_handle: AppHandle) -> bool {
    // Check if permission is granted
    if app_handle
        .android_permission()
        .check_permissions()
        .expect("Failed to check read audio permission")
        .read_media_audio != PermissionType::Granted {
        // Request permission and return the result
        return app_handle
            .android_permission()
            .request_permissions(Some(vec![PermissionType::ReadMediaAudio]))
            .unwrap()
            .audio
            == PermissionState::Granted;
    }
    true
}
```
