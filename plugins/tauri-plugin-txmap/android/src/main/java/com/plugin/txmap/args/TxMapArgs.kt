package com.plugin.txmap.com.plugin.txmap.args

import app.tauri.annotation.InvokeArg
import app.tauri.plugin.Channel
@InvokeArg
class Task(
    val id: String = "",
    val status: String = "",
    val startPosition: Position? = Position(0f,0f),
    val endPosition: Position? = Position(0f,0f),
    val sendInfo: UserInfo? = null,
    val receiveInfo: UserInfo? = null
) {
    class Position(
        val lat: Float?,
        val lng: Float?
    )

    class UserInfo(
        val avatar: String? = ""
    )
}
@InvokeArg
class TxMapOptions {
    var tasks: HashMap<String, Task>? = HashMap()
}

@InvokeArg
class TxMapArgs {
    val options: TxMapOptions? = TxMapOptions()
    lateinit var channel: Channel
}

@InvokeArg
class AddTaskArgs {
    val task: Task? = Task()
    val preview: Boolean? = true
}

@InvokeArg
class WatchOptions {
    val watch: Boolean? = false
    val timeout: Long? = 3000   // ms
    val rate: Long? = 3000      // ms
}

@InvokeArg
class WatchPositionArgs {
    val options: WatchOptions? = WatchOptions()
    lateinit var channel: Channel
}

@InvokeArg
class ClearWatchLocationArgs {
    var channelId: Long = 0
}