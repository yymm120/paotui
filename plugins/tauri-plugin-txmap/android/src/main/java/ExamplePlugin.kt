package com.plugin.txmap

import android.app.Activity
import android.content.Context.LOCATION_SERVICE
import android.content.Intent
import android.content.Intent.FLAG_ACTIVITY_REORDER_TO_FRONT
import android.content.pm.ActivityInfo
import android.location.Location
import android.location.LocationManager
import android.os.Build
import android.util.Log
import android.webkit.WebView
import androidx.activity.ComponentActivity
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.ViewModelProvider
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import com.plugin.txmap.com.plugin.txmap.args.FragmentArgs
import com.plugin.txmap.com.plugin.txmap.args.NavigationArgs
import com.plugin.txmap.com.plugin.txmap.args.TxMapArgs
import com.plugin.txmap.navi.WalkGpsNavActivity
import com.tencent.map.geolocation.TencentLocationManager

import com.plugin.txmap.com.plugin.txmap.args.AddTaskArgs
import com.plugin.txmap.com.plugin.txmap.args.ClearWatchLocationArgs
import com.plugin.txmap.com.plugin.txmap.args.WatchPositionArgs
import com.plugin.txmap.com.plugin.txmap.util.FailedCallback
import com.plugin.txmap.com.plugin.txmap.util.LocationUtil
import com.plugin.txmap.com.plugin.txmap.util.SuccessCallback
import kotlin.getValue

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example()
    private var webView: WebView? = null
    private var locationManager: TencentLocationManager? = null
    private lateinit var locationUtil: LocationUtil
    private var watchers = hashMapOf<Long, Pair<Invoke, WatchPositionArgs>>()

    private val viewModel: TaskViewModel by lazy {
        ViewModelProvider(activity as ComponentActivity).get(TaskViewModel::class.java)
    }

    override fun load(webView: WebView) {
        super.load(webView)
        this.webView = webView
        locationUtil = LocationUtil(context = activity.applicationContext)
    }


    override fun onResume() {
        super.onResume()
        for ((invoke, args) in watchers.values) {
            startWatch(invoke, args)
        }
    }


    override fun onNewIntent(intent: Intent) {
        // 处理重启事件

    }

    override fun onPause() {
        super.onPause()
        locationUtil.clearUpdates()
    }



    fun displayFragment(activity: AppCompatActivity) {
        activity.requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_PORTRAIT
        activity
            .supportFragmentManager
            .beginTransaction()
            .setReorderingAllowed(true)
            .add(android.R.id.content, BlankFragment.newInstance("1", "2"))
            .commit();
    }


    fun startDemoActivity() {
        val routeIntent: Intent = Intent(activity, WalkGpsNavActivity::class.java)
        activity.startActivity(routeIntent)
    }


    @Command
    fun start_listening() {
        // 监听来自 JS 的事件

    }

    /**
     * 开启并默认显示Fragment
     */
    @Command
    fun fragment(invoke: Invoke) {
        var args = invoke.parseArgs(FragmentArgs::class.java)
        if (args.close == true) {
            Log.d("fragment command", "close");
            // 彻底关闭
            val ret = JSObject()
            ret.put("fragment_status", "closed")
            return invoke.resolve(ret)
        }
//        startDemoActivity()

//        displayFragment(activity as AppCompatActivity)

        BlankFragment.newInstance("1", "2").show((activity as AppCompatActivity).supportFragmentManager, "dialog")
    }


    /**
     * 开始导航
     */
    @Command
    fun startNavigation(invoke: Invoke) {

        // 启动导航页面
        var args = invoke.parseArgs(NavigationArgs::class.java)
        val intent: Intent = Intent(activity, WalkGpsNavActivity::class.java).apply {
            addFlags(FLAG_ACTIVITY_REORDER_TO_FRONT)
        }
        activity.startActivity(intent)

        val locationManager = activity.getSystemService(LOCATION_SERVICE) as LocationManager;

        // 获取当前经纬度, 获取目标经纬度
        val providers = locationManager.getProviders(true)
        var bestLocation = null
        for (provider in providers) {
            val l = locationManager.getLastKnownLocation(provider);
            if (l == null) {
                continue;
            }
        }
    }

    fun successCallback(location: Location): Unit {
        return Unit
    }

    fun failedCallback(msg: String) : Unit {
        return Unit
    }

    @Command
    fun watchLocation(invoke: Invoke) {
        val args = invoke.parseArgs(WatchPositionArgs::class.java)

        Log.d("into watch", args.toString())

        startWatch(invoke, args)
    }

    fun startWatch(invoke: Invoke, args: WatchPositionArgs) {
        locationUtil.locate(
            args.options!!.watch,
            args.options.timeout,
            args.options.rate,
            successCallback = { location -> args.channel.send(convertLocation(location)) },
            failedCallback = { error -> args.channel.sendObject(error) }
        );
        watchers[args.channel.id] = Pair(invoke, args)
    }


    @Command
    fun clearWatchLocation(invoke: Invoke) {
        val args = invoke.parseArgs(ClearWatchLocationArgs::class.java)

        watchers.remove(args.channelId)

        if (watchers.isEmpty()) {
            locationUtil.clearUpdates()
        }

        invoke.resolve()
    }


    @Command
    fun clearWatchDirection(invoke: Invoke) {
        Log.d("into", "clearWatchDirection");
        invoke.resolve()
    }

    @Command
    fun watchDirection(invoke: Invoke) {
        Log.d("into", "watchDirection");
        invoke.resolve()
    }


    @Command
    fun startTxMapWithChannel(invoke: Invoke) {
        var args = invoke.parseArgs(TxMapArgs::class.java)
        Log.d("into", "startTxMapWithChannel: ${invoke.getArgs()}")

        // 共享数据
        args.options?.let { options ->
            viewModel.updateData(options)
        }

        // 启动地图
        BlankFragment.newInstance("1", "2").show((activity as AppCompatActivity).supportFragmentManager, "dialog")
    }

    @Command
    fun addTask(invoke: Invoke) {
        var args = invoke.parseArgs(AddTaskArgs::class.java)
        Log.d("into", "addTask ${args.task}")

        args.task?.let { task ->
            Log.d("listener", "11111111111111")
            viewModel.addTask(task)
        }
    }

    @Command
    fun clearTask(invoke: Invoke) {
        Log.d("into", "clearTask")
    }

    @Command
    fun clearChannel(invoke: Invoke) {
        (activity as ComponentActivity).viewModelStore.clear()
        Log.d("into", "clearChannel")
    }

    private fun convertLocation(location: Location): JSObject {
        val ret = JSObject()
        val coords = JSObject()

        coords.put("latitude", location.latitude)
        coords.put("longitude", location.longitude)
        coords.put("accuracy", location.accuracy)
        coords.put("altitude", location.altitude)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            coords.put("altitudeAccuracy", location.verticalAccuracyMeters)
        }
        coords.put("speed", location.speed)
        coords.put("heading", location.bearing)
        ret.put("timestamp", location.time)
        ret.put("coords", coords)

        return ret
    }
}

