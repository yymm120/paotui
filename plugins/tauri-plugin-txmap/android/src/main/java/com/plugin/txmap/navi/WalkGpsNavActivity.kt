package com.plugin.txmap.navi

import android.os.Bundle
import android.util.Pair
import androidx.appcompat.app.AppCompatActivity
import com.plugin.txmap.R
import com.tencent.map.geolocation.TencentLocation
import com.tencent.map.geolocation.TencentLocationListener
import com.tencent.map.geolocation.TencentLocationManager
import com.tencent.map.geolocation.TencentLocationRequest
import com.tencent.navix.api.NavigatorConfig
import com.tencent.navix.api.NavigatorZygote
import com.tencent.navix.api.config.SimulatorConfig
import com.tencent.navix.api.layer.NavigatorLayerRootWalk
import com.tencent.navix.api.layer.NavigatorViewStub
import com.tencent.navix.api.model.NavNonMotorRoute
import com.tencent.navix.api.model.NavRouteReqParam
import com.tencent.navix.api.model.NavSearchPoint
import com.tencent.navix.api.navigator.NavigatorWalk
import com.tencent.navix.api.observer.SimpleNavigatorWalkObserver
import com.tencent.navix.api.plan.RoutePlanRequestCallback
import com.tencent.navix.api.plan.RoutePlanRequester
import com.tencent.navix.tts.api.TTSPlayer
import com.tencent.navix.ui.NavigatorLayerViewWalk
import com.tencent.tencentmap.mapsdk.maps.TencentMapInitializer

class WalkGpsNavActivity : AppCompatActivity(), TencentLocationListener {
    private var navigatorWalk: NavigatorWalk? = null
    private var layerRootWalk: NavigatorLayerRootWalk? = null
    private var layerViewWalk: NavigatorLayerViewWalk? = null
    private var locationManager: TencentLocationManager? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_walking)

        locationManager = TencentLocationManager.getInstance(this.applicationContext, Pair("key1", "value1"));

        val request = TencentLocationRequest.create()
        request.setInterval(10000);
        request.setRequestLevel(TencentLocationRequest.REQUEST_LEVEL_ADMIN_AREA);
        request.setAllowGPS(true)
        request.setAllowDirection(true)

        val locMode: Int = TencentLocationRequest.REQUEST_LEVEL_GEO
        request.setAllowGPS(true)
        locationManager?.requestLocationUpdates(request,this, 0);

        // 设置同意地图SDK隐私协议
        TencentMapInitializer.setAgreePrivacy(applicationContext, true)
        TencentMapInitializer.start(applicationContext)


        // 初始化导航SDK
        NavigatorZygote.with(applicationContext).init(
            NavigatorConfig.builder() // 设置同意导航SDK隐私协议
                .setUserAgreedPrivacy(true) // 设置自定义的可区分设备的ID
                .setDeviceId("develop_custom_id_123456") //                        .experiment().setDebug(true)
                .build()
        )
        NavigatorZygote.with(applicationContext).start()

        navigatorWalk = NavigatorZygote.with(applicationContext).navigator(
            NavigatorWalk::class.java
        )
        val navigatorViewStub = findViewById<NavigatorViewStub>(R.id.view_stub)
        navigatorViewStub.setTravelMode(NavRouteReqParam.TravelMode.TravelModeWalking)
        navigatorViewStub.inflate()
        layerRootWalk = navigatorViewStub.getNavigatorView()
        navigatorWalk?.bindView(layerRootWalk)

        layerViewWalk = NavigatorLayerViewWalk(applicationContext)
        layerRootWalk?.addViewLayer(layerViewWalk)
        navigatorWalk?.registerObserver(observer)
        //算路开导航
        navigatorWalk?.searchRoute(
            RoutePlanRequester.Companion.newBuilder(NavRouteReqParam.TravelMode.TravelModeWalking)
                .start(NavSearchPoint(40.007056, 116.389895))
                .end(NavSearchPoint(39.513005, 116.416642))
                .build(), RoutePlanRequestCallback<NavNonMotorRoute> { navRoutePlan, navError ->
                val routeDatas = navRoutePlan.routes
                val routeId = routeDatas[0].getRouteId()
                navigatorWalk?.simulator()?.setEnable(true)?.setConfig(
                    SimulatorConfig.builder(SimulatorConfig.Type.SIMULATE_LOCATIONS_ALONG_ROUTE)
                        .setSimulateSpeed(6).build()
                )
                navigatorWalk?.startNavigation(routeId)
            })
    }

    override fun onStart() {
        super.onStart()
        layerRootWalk!!.onStart()
    }

    override fun onRestart() {
        super.onRestart()
        layerRootWalk!!.onRestart()
    }

    override fun onResume() {
        super.onResume()
        layerRootWalk!!.onResume()
    }

    override fun onPause() {
        super.onPause()
        layerRootWalk!!.onPause()
        navigatorWalk!!.stopNavigation()
    }

    override fun onStop() {
        super.onStop()
        layerRootWalk!!.onStop()
    }

    override fun onDestroy() {
        super.onDestroy()
        // 移除导航监听
        navigatorWalk!!.unregisterObserver(observer)
        // 移除默认面板
        layerRootWalk!!.removeViewLayer(layerViewWalk)
        // 解绑导航地图
        navigatorWalk!!.unbindView(layerRootWalk)

        // 关闭TTS和导航
        val ttsPlayer = navigatorWalk!!.ttsPlayer
        if (ttsPlayer is TTSPlayer) {
            (ttsPlayer as TTSPlayer).stop()
        }
        navigatorWalk!!.stopNavigation()
        layerRootWalk!!.onDestroy()
    }

    private val observer: SimpleNavigatorWalkObserver = object : SimpleNavigatorWalkObserver() {
        //即将到达目的地
        override fun onWillArriveDestination() {
            super.onWillArriveDestination()
        }
    }

    override fun onLocationChanged(p0: TencentLocation?, p1: Int, p2: String?) {
        TODO("Not yet implemented")
    }

    override fun onStatusUpdate(p0: String?, p1: Int, p2: String?) {
        TODO("Not yet implemented")
    }

    override fun onGnssInfoChanged(p0: Any?) {
        TODO("Not yet implemented")
    }

    override fun onNmeaMsgChanged(p0: String?) {
        TODO("Not yet implemented")
    }
}