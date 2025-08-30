package com.plugin.txmap.com.plugin.txmap.navi

import android.content.Context
import android.graphics.BitmapFactory
import android.graphics.Color
import android.util.AttributeSet
import android.view.LayoutInflater
import android.widget.LinearLayout
import com.plugin.txmap.BlankFragment.TailwindUnitConverter.spacingToDp
import com.plugin.txmap.R
import com.tencent.navix.api.map.MapApi
import com.tencent.navix.api.model.NavRoutePlan
import com.tencent.tencentmap.mapsdk.maps.CameraUpdateFactory
import com.tencent.tencentmap.mapsdk.maps.model.BitmapDescriptorFactory
import com.tencent.tencentmap.mapsdk.maps.model.LatLng
import com.tencent.tencentmap.mapsdk.maps.model.LatLngBounds
import com.tencent.tencentmap.mapsdk.maps.model.Marker
import com.tencent.tencentmap.mapsdk.maps.model.MarkerOptions
import com.tencent.tencentmap.mapsdk.maps.model.OverlayLevel
import com.tencent.tencentmap.mapsdk.maps.model.Polyline
import com.tencent.tencentmap.mapsdk.maps.model.PolylineOptions

class RouteView(context: Context, attrs: AttributeSet?) : LinearLayout(context, attrs) {
    private lateinit var tencentMap: MapApi
    private var plan: NavRoutePlan<*>? = null

    init {
        LayoutInflater.from(context).inflate(R.layout.view_route, this)
    }

    fun updateRoutePlan(routePlan: NavRoutePlan<*>?) {
        plan = routePlan
        drawRoute()
    }

    fun clear() {
        // 清除线条
        polylineMap.forEach {
            it.value.remove()
        }
        polylineMap.clear()
        // 清除起始点/终止点图标
        startMarker?.remove()
        startMarker = null
        endMarker?.remove()
        endMarker = null
    }


    private val polylineMap = mutableMapOf<Int, Polyline>()
    private var startMarker: Marker? = null
    private var endMarker: Marker? = null

    private fun drawRoute() {

        clear()

        plan?.apply {
            startMarker = tencentMap.addMarker(
                MarkerOptions(LatLng(startPoi.latitude, startPoi.longitude))
                    .icon(BitmapDescriptorFactory.fromBitmap(BitmapFactory.decodeResource(resources, R.drawable.app_icon_start_point)))
            )
            endMarker = tencentMap.addMarker(
                MarkerOptions(LatLng(endPoi.latitude, endPoi.longitude))
                    .icon(BitmapDescriptorFactory.fromBitmap(BitmapFactory.decodeResource(resources, R.drawable.app_icon_end_point)))
            )
        }

        val builder = LatLngBounds.Builder()

        plan?.routes?.forEachIndexed { index, routeData ->

            val zIndex = 100
            val indexes = intArrayOf(0, routeData.routePoints.size)
//            var colors = intArrayOf(ROUTE_COLOR_BACKUP, ROUTE_COLOR_BACKUP)
//            var borderColors = intArrayOf(ROUTE_COLOR_BACKUP_STROKE, ROUTE_COLOR_BACKUP_STROKE)


            builder.include(routeData.routePoints)

            val options = PolylineOptions()
            options.addAll(routeData.routePoints)
                .arrow(true)
                .arrowTexture(BitmapDescriptorFactory.fromBitmap(BitmapFactory.decodeResource(resources, R.drawable.app_arrow_texture)))
                .color(Color.GREEN)
                .lineType(0)
                .arrowSpacing(150)
                .zIndex(zIndex)
                .level(OverlayLevel.OverlayLevelAboveBuildings)
                .width(32f)
                .clickable(true)
                .borderWidth(4f)
//                .borderColors(borderColors)
//                .colors(colors, indexes)

            polylineMap[index] = tencentMap.addPolyline(options)
        }

        val padding = 2f.spacingToDp(context)
        tencentMap.moveCamera(
            CameraUpdateFactory.newLatLngBoundsRect(builder.build(),
            padding, padding,padding,padding,
        ))
    }
    fun injectMap(map: MapApi) {
        tencentMap = map
    }
}