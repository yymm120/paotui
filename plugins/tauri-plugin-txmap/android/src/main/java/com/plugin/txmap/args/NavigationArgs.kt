package com.plugin.txmap.com.plugin.txmap.args

import app.tauri.annotation.InvokeArg
import app.tauri.plugin.Channel

@InvokeArg
class NavigationArgs {
    lateinit var channel: Channel
}