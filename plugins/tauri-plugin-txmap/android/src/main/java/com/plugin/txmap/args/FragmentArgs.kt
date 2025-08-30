package com.plugin.txmap.com.plugin.txmap.args

import app.tauri.annotation.InvokeArg

@InvokeArg
class FragmentArgs {
    var headless: Boolean? = false;
    var close: Boolean? = false;
}

