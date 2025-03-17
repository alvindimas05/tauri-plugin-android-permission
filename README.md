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
pub fn check_read_audio_permission() -> bool {
    let app_handle = GLOBAL_APP_HANDLE.get().unwrap();
    if app_handle
        .android_permission()
        .check_permissions()
        .expect("Failed to request read audio permission")
        .audio
        != PermissionState::Granted
    {
        return app_handle
            .android_permission()
            .request_permissions(Some(vec![PermissionType::ReadMediaAudio]))
            .unwrap()
            .audio
            == PermissionState::Granted;
    }
    true
}

#[tauri::command]
pub fn request_read_audio_permission() -> bool {
    let app_handle = GLOBAL_APP_HANDLE.get().unwrap();
    if check_read_audio_permission() {
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
