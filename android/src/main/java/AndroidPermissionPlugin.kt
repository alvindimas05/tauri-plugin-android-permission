package org.plugin.androidpermission

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin

private const val ALIAS_READ_AUDIO: String = "readMediaAudio"
private const val ALIAS_READ_IMAGES: String = "readMediaImages"
private const val ALIAS_READ_VIDEO: String = "readMediaVideo"
private const val ALIAS_READ_EXTERNAL_STORAGE: String = "readExternalStorage"
private const val ALIAS_WRITE_EXTERNAL_STORAGE: String = "writeExternalStorage"
@TauriPlugin(
    permissions = [
        Permission(strings = [
            Manifest.permission.READ_MEDIA_AUDIO
        ],
            alias = ALIAS_READ_AUDIO
        ),
        Permission(strings = [
            Manifest.permission.READ_MEDIA_IMAGES
        ],
            alias = ALIAS_READ_IMAGES
        ),
        Permission(strings = [
            Manifest.permission.READ_MEDIA_VIDEO
        ],
            alias = ALIAS_READ_VIDEO
        ),
        Permission(strings = [
            Manifest.permission.READ_EXTERNAL_STORAGE
        ],
            alias = ALIAS_READ_EXTERNAL_STORAGE
        ),
        Permission(strings = [
            Manifest.permission.READ_EXTERNAL_STORAGE
        ],
            alias = ALIAS_WRITE_EXTERNAL_STORAGE
        )
    ]
)

@TauriPlugin
class AndroidPermissionPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = AndroidPermission()
}
