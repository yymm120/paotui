package com.plugin.txmap

import android.app.Dialog
import android.content.Context
import android.os.Bundle
import android.util.Log
import android.util.TypedValue
import android.view.Gravity
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.view.Window
import android.view.WindowManager
import android.widget.FrameLayout
import androidx.fragment.app.DialogFragment
import androidx.fragment.app.activityViewModels
import androidx.lifecycle.Observer
import com.plugin.txmap.BlankFragment.TailwindUnitConverter.spacingToDp
import com.plugin.txmap.com.plugin.txmap.args.Task
import com.plugin.txmap.com.plugin.txmap.navi.RouteView
import com.tencent.map.geolocation.TencentLocationManager
import com.tencent.navix.api.NavigatorConfig
import com.tencent.navix.api.NavigatorZygote
import com.tencent.navix.api.layer.NavigatorLayerRootWalk
import com.tencent.navix.api.layer.NavigatorViewStub
import com.tencent.navix.api.model.NavMode
import com.tencent.navix.api.model.NavNonMotorRoute
import com.tencent.navix.api.model.NavRouteReqParam
import com.tencent.navix.api.model.NavSearchPoint
import com.tencent.navix.api.navigator.NavigatorWalk
import com.tencent.navix.api.observer.SimpleNavigatorWalkObserver
import com.tencent.navix.api.plan.RoutePlanRequestCallback
import com.tencent.navix.api.plan.RoutePlanRequester
import com.tencent.navix.ui.NavigatorLayerViewWalk
import com.tencent.navix.ui.api.NavigatorLayerViewApi
import com.tencent.navix.ui.api.NavigatorLayerViewDriveApi
import com.tencent.navix.ui.api.config.UIComponentConfig
import com.tencent.tencentmap.mapsdk.maps.MapView
import com.tencent.tencentmap.mapsdk.maps.TencentMapInitializer
import kotlin.math.roundToInt

private const val ARG_PARAM1 = "param1"
private const val ARG_PARAM2 = "param2"


class BlankFragment : DialogFragment() {
    // TODO: Rename and change types of parameters
    private var param1: String? = null
    private var param2: String? = null

    private var navigatorWalk: NavigatorWalk? = null
    private var layerRootWalk: NavigatorLayerRootWalk? = null
    private var layerViewWalk: NavigatorLayerViewWalk? = null
    private var locationManager: TencentLocationManager? = null

    private var routeView: RouteView? = null
    private val viewModel: TaskViewModel by activityViewModels()
    private var mapView: MapView? = null;

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        arguments?.let {
            param1 = it.getString(ARG_PARAM1)
            param2 = it.getString(ARG_PARAM2)
        }

        Log.i("into onCreate", "----------------------")
    }

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        // Inflate the layout for this fragment

        Log.i("into onCreateView", "----------------------")
        // 确保根布局不拦截事件
        view?.isClickable = false




        return inflater.inflate(R.layout.fragment_blank, container, false)
    }

    override fun onCreateDialog(savedInstanceState: Bundle?): Dialog {
        // 重写此方法以设置对话框样式
        val dialog = super.onCreateDialog(savedInstanceState)

        // 2. 隐藏标题栏（可选）
        dialog.requestWindowFeature(Window.FEATURE_NO_TITLE)

        // 3. 禁用点击外部消失的行为
        dialog.setCanceledOnTouchOutside(false)

        return dialog
    }

    override fun onStart() {
        super.onStart()
        // 设置对话框宽度占屏幕80%，高度自适应
        dialog?.window?.apply {

            // 获取屏幕尺寸
            val displayMetrics = resources.displayMetrics
            val screenWidth = displayMetrics.widthPixels
            val screenHeight = displayMetrics.heightPixels

            setDimAmount(0f) // 0f 表示完全透明
            clearFlags(WindowManager.LayoutParams.FLAG_DIM_BEHIND)
            setBackgroundDrawableResource(android.R.color.transparent)
            setLayout(
                WindowManager.LayoutParams.MATCH_PARENT,
                WindowManager.LayoutParams.MATCH_PARENT,
            )

            // 调整窗口参数
            val params = attributes.apply {
                flags = WindowManager.LayoutParams.FLAG_NOT_TOUCH_MODAL  // 关键！
                // 设置窗口宽度和高度
                width = (screenWidth).toInt()
                height = (screenHeight * 0.4).toInt() // 40%高度

                gravity = Gravity.TOP or Gravity.START // 从左上角开始计算
                x = 0
                y = 12f.spacingToDp(requireContext())
            }

            attributes = params
        }
        // 设置对话框背景透明（确保圆角生效）
//        dialog?.window?.setBackgroundDrawableResource(android.R.color.transparent)

        startNavigation()
    }

    fun startNavigation() {
        // 设置同意地图SDK隐私协议
        TencentMapInitializer.setAgreePrivacy(requireContext(), true)
        TencentMapInitializer.start(requireContext())


        // 初始化导航SDK
        NavigatorZygote.with(requireContext()).init(
            NavigatorConfig.builder() // 设置同意导航SDK隐私协议
                .setUserAgreedPrivacy(true) // 设置自定义的可区分设备的ID
                .setDeviceId("develop_custom_id_123456") //                        .experiment().setDebug(true)
                .build()
        )
        NavigatorZygote.with(requireContext()).start()

        navigatorWalk = NavigatorZygote.with(requireContext()).navigator(
            NavigatorWalk::class.java
        )

        val navigatorViewStub = view?.findViewById<NavigatorViewStub>(R.id.view_stub1)
        navigatorViewStub?.setTravelMode(NavRouteReqParam.TravelMode.TravelModeWalking)
        navigatorViewStub?.inflate()
        layerRootWalk = navigatorViewStub?.getNavigatorView()
        navigatorWalk?.bindView(layerRootWalk)

        layerViewWalk = NavigatorLayerViewWalk(requireContext())
        layerRootWalk?.addViewLayer(layerViewWalk)
        val walkUiController = layerViewWalk as NavigatorLayerViewApi<*>
        layerRootWalk?.navMode = NavMode.MODE_OVERVIEW

//        layerRootWalk?.mapApi.

        // 配置UI组件
        walkUiController.uiComponentConfig = UIComponentConfig.builder()
            .setComponentVisibility(UIComponentConfig.UIComponent.INFO_VIEW, true)
            .setComponentVisibility(UIComponentConfig.UIComponent.BOTTOM_PANEL_VIEW, false)
            .build()

        navigatorWalk?.registerObserver(observer)

        val taskMap: HashMap<String, Task>? = null
        // 获取共享的ViewModel
        viewModel.sharedData.observe(viewLifecycleOwner, Observer { taskOptions ->
            Log.d("listener", "view model ${taskOptions.tasks}")
            // 更新标记
            taskOptions.tasks;
        })

        initRouteView()
//        layerViewWalk!!.addView(routeView)

        mapView = MapView(requireContext())

        layerViewWalk?.addView(MapView(requireContext()))
        //算路开导航
//        navigatorWalk?.searchRoute(
//            RoutePlanRequester.Companion.newBuilder(NavRouteReqParam.TravelMode.TravelModeWalking)
//                .start(NavSearchPoint(40.007056, 116.389895))
//                .end(NavSearchPoint(40.013005, 116.316642))
//                .build(),
//            RoutePlanRequestCallback<NavNonMotorRoute> { navRoutePlan, navError ->
//                if (navError != null) {
//                    // handle error
//                    return@RoutePlanRequestCallback
//                }
//                if (navRoutePlan != null) {
//
////                    routeView?.updateRoutePlan(navRoutePlan)
//                    // handle result
////                    routeView.updateRoutePlan(navRoutePlan)
//
//                }
//
//            })
    }

    private fun initRouteView() {
        routeView = RouteView(requireContext(), null)
        routeView?.injectMap(layerRootWalk?.getMapApi()!!)
    }

    private val observer: SimpleNavigatorWalkObserver = object : SimpleNavigatorWalkObserver() {
        //即将到达目的地
        override fun onWillArriveDestination() {
            super.onWillArriveDestination()
        }
    }

    companion object {

        @JvmStatic
        fun newInstance(param1: String, param2: String) =
            BlankFragment().apply {
                arguments = Bundle().apply {
                    putString(ARG_PARAM1, param1)
                    putString(ARG_PARAM2, param2)
                }
            }
    }

    // 理一下逻辑
    fun demo() {
        // 1. 启动地图
        // 2. 定位到当前位置

    }


    fun setupTencentMapNavigationUI(layerViewDrive: NavigatorLayerViewDriveApi) {
    // 1. UI组件配置
//    UIComponentConfig uiConfig = UIComponentConfig.builder()
//            .setAllComponentVisibility(false)
//            .setComponentVisibility(UIComponentConfig.UIComponent.INFO_VIEW, true)
//            .setComponentVisibility(UIComponentConfig.UIComponent.GUIDE_LANE_VIEW, true)
//            .setComponentVisibility(UIComponentConfig.UIComponent.BOTTOM_PANEL_VIEW, true)
//            .setComponentVisibility(UIComponentConfig.UIComponent.SPEED_VIEW_VIEW, true)
//            .setComponentVisibility(UIComponentConfig.UIComponent.ROAD_LIMIT_VIEW, true)
//            .setComponentVisibility(UIComponentConfig.UIComponent.TRAFFIC_BAR_VIEW, true)
//            .setComponentVisibility(UIComponentConfig.UIComponent.PREVIEW_SWITCH_VIEW, true)
//            .setComponentVisibility(UIComponentConfig.UIComponent.REROUTE_SWITCH_VIEW, true)
//            .build();
//
//    layerViewDrive.setUIComponentConfig(uiConfig);
//
//    // 2. 边距配置
//    ViewMarginConfig marginConfig = new ViewMarginConfig();
//    marginConfig.setInfoViewMargin(50, 50, 50, 50);
//    marginConfig.setSpeedViewMargin(0, 100, 50, 0);
//    marginConfig.setBottomPanelMargin(0, 0, 0, 30);
//
//    layerViewDrive.setViewMarginConfig(marginConfig);
//
//    // 3. 视野配置
//    NavMapVisionConfig visionConfig = new NavMapVisionConfig();
//    visionConfig.setProportion(0.3f);
//    visionConfig.setAutoChangeWhenGuide(true);
//    visionConfig.setCarPositionRatio(0.3f, 0.5f);
//
//    layerViewDrive.setNavMapVisionConfig(visionConfig);
//
//    // 4. 放大图配置
//    EnlargedMapUIConfig enlargedConfig = new EnlargedMapUIConfig();
//    enlargedConfig.setEnlargeMapSwitch(true);
//    enlargedConfig.setEnlargeMapAutoSwitch(true);
//    enlargedConfig.setEnlargeMapMargin(30, 30, 30, 30);
//
//    layerViewDrive.setEnlargedMapUIConfig(enlargedConfig);
}

    object TailwindUnitConverter {

        /**
         * 将 Tailwind 的 rem 转换为 dp（用于布局、边距等）
         * @param rem Tailwind 中的 rem 值（默认 1rem = 16px）
         * @param rootFontSizePx Web 根字体大小（默认 16px）
         */
        fun Float.remToDp(context: Context, rootFontSizePx: Int = 16): Int {
            val px = this * rootFontSizePx
            return TypedValue.applyDimension(
                TypedValue.COMPLEX_UNIT_DIP,
                px,
                context.resources.displayMetrics
            ).roundToInt()
        }

        /**
         * 将 Tailwind 的 rem 转换为 sp（用于字体大小）
         * @param rem Tailwind 中的 rem 值
         * @param rootFontSizePx Web 根字体大小（默认 16px）
         */
        fun Float.remToSp(context: Context, rootFontSizePx: Int = 16): Int {
            val px = this * rootFontSizePx
            return TypedValue.applyDimension(
                TypedValue.COMPLEX_UNIT_SP,
                px,
                context.resources.displayMetrics
            ).roundToInt()
        }

        fun Float.spacingToDp(context: Context, rootFontSizePx: Int = 16) : Int {
            val px = this * rootFontSizePx * 0.25
            return TypedValue.applyDimension( // deriveDimension 需要API 34以上, 用applyDimension替代
                TypedValue.COMPLEX_UNIT_DIP,
                px.toFloat(),
                context.resources.displayMetrics
            ).roundToInt()
        }

        fun Float.spacingToSp(context: Context, rootFontSizePx: Int = 16) : Int {
            val px = this * rootFontSizePx * 0.25
            return TypedValue.applyDimension( // deriveDimension 需要API 34以上, 用applyDimension替代
                TypedValue.COMPLEX_UNIT_SP,
                px.toFloat(),
                context.resources.displayMetrics
            ).roundToInt()
        }
    }

}


