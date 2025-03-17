package org.plugin.android-permission

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin

private const val ALIAS_READ_AUDIO: String = "audio"
private const val ALIAS_READ_IMAGES: String = "images"
private const val ALIAS_READ_VIDEO: String = "video"
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
        )
    ]
)

@TauriPlugin
class AndroidPermissionPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = AndroidPermission()
}
