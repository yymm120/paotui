package com.plugin.txmap.com.plugin.txmap.util

import android.content.Context
import android.location.Location
import android.location.LocationListener
import android.location.LocationManager

import android.os.SystemClock
import android.util.Log
import androidx.core.location.LocationManagerCompat
import com.google.android.gms.common.ConnectionResult
import com.google.android.gms.common.GoogleApiAvailability
import com.google.android.gms.location.FusedLocationProviderClient
import com.google.android.gms.location.LocationRequest
import com.google.android.gms.location.LocationServices
import com.google.android.gms.location.Priority



typealias SuccessCallback = (Location) -> Unit
typealias FailedCallback = (String) -> Unit
interface AnyListener: com.google.android.gms.location.LocationListener, LocationListener {}


/**
 * 定位工具类
 */
class LocationUtil(
  private val context: Context
): AnyListener {

  // Location管理类, 提供requestLocationUpdates等方法异步获取Location坐标
  private val locationManager: LocationManager = context.getSystemService(Context.LOCATION_SERVICE) as LocationManager
  // Google 定位 Client
  private var fusedLocationClient: FusedLocationProviderClient? = null
  // 是否是单次定位
  private var isSingleLocate = true;
  // 获取Location坐标的超时时间
  private var timeout: Long = 3000 // ms
  // 是否是以监听的方式异步获取Location坐标
  private var watch: Boolean = true
  // 异步获取Location坐标的速率, 也是lastLocation的允许的最大缓存时间
  private var rate: Long = 3000
  private lateinit var successCallback: SuccessCallback
  private lateinit var failedCallback: FailedCallback



  /**
   * 获取上一次的定位结果, 如果有就返回, 如果没有就获得最新的地址
   */
  private fun getLastLocation(maxAge: Long = 3000): Location? {
    this.rate = maxAge
    var lastLocation: Location? = null
    val locationManager = context.getSystemService(Context.LOCATION_SERVICE) as LocationManager

    for (provider in locationManager.allProviders) {
      val tempLocation = locationManager.getLastKnownLocation(provider)
      if (tempLocation != null) {
        // 系统时间 - 地址更新时间 = 地址缓存时间
        val locationCachedAge = SystemClock.elapsedRealtimeNanos() - tempLocation.elapsedRealtimeNanos
        val maxAgeNano = rate * 1000000L // ns -> ms
        if (locationCachedAge <= maxAgeNano && (lastLocation == null || lastLocation.elapsedRealtimeNanos > tempLocation.elapsedRealtimeNanos)) {
          lastLocation = tempLocation
        }
      }
    }
    return lastLocation
  }



  /**
   * 获取最新的地址
   */
  private fun watchCurrentLocationFromGPS(): Boolean {
    // 请求GPS定位更新（参数说明↓）
    Log.e("location gps", "into")
    try {
      if (this.watch) {
        Log.d("location gps", "watch mode")
        locationManager.requestLocationUpdates(
          LocationManager.GPS_PROVIDER, // 使用GPS定位
          this.timeout,          // 最小时间间隔（毫秒）
          1f,       // 最小距离变化（米）
          this            // 当gps数据更新, 调用onLocationChanged方法
        )
      } else {
        Log.d("location gps", "single mode")
        locationManager.requestLocationUpdates(
          LocationManager.GPS_PROVIDER, // 使用GPS定位
          0,          // 最小时间间隔（毫秒）
          0f,       // 最小距离变化（米）
          this            // 当gps数据更新, 调用onLocationChanged方法
        )
      }
    } catch (e: Exception) {
      Log.e("location error", e.message.toString())
      return false
    }

    return true
  }



  /**
   * 访问腾讯定位, 连续获取最新的位置, 频率默认为3s
   */
  private fun watchCurrentLocationFromTencent(): Boolean {
    var currentLocation : Location? = null;
    try {
      Log.e("location tencent", "todo")

    } catch (e: Exception) {
      Log.e("location error", e.message.toString())
      return false
    }
    return false
  }


  private fun watchCurrentLocationFromGoogle(): Boolean {

    Log.e("location google", "into")
    val resultCode = GoogleApiAvailability.getInstance().isGooglePlayServicesAvailable(context);
    val isLocationServicesEnabled = LocationManagerCompat.isLocationEnabled(locationManager)
    if (resultCode != ConnectionResult.SUCCESS || !isLocationServicesEnabled) {
      Log.e("location google", "unavailable")
      return false
    }

    try {
      fusedLocationClient = LocationServices.getFusedLocationProviderClient(context)
      var locationRequest: LocationRequest? = null
      locationRequest = if (this.watch) {
        LocationRequest.Builder(this.rate) // 间隔
          .setMaxUpdateDelayMillis(this.timeout) // 两次请求间隔的最大时间
          .setMinUpdateIntervalMillis(this.rate) // 两次请求间隔的最小时间
          .setPriority(Priority.PRIORITY_HIGH_ACCURACY)
          .build()
      } else {
        LocationRequest.Builder(0) // 间隔
          .setMaxUpdateDelayMillis(0) // 两次请求间隔的最大时间
          .setMinUpdateIntervalMillis(0) // 两次请求间隔的最小时间
          .setPriority(Priority.PRIORITY_HIGH_ACCURACY)
          .build()
      }

      fusedLocationClient?.requestLocationUpdates(locationRequest, this, null)
    } catch (e: Exception) {
      Log.e("location error", e.message.toString())
      return false
    }
    return true
  }



  private fun watchCurrentLocationAnyway(): Boolean {
    Log.e("location anyway", "todo")
    return false
  }


  fun clearUpdates() {
    locationManager.removeUpdates(this)
  }
  /**
   * 定位策略:
   *  1. 单次定位: 先访问上一次缓存的地址, 缓存过期后, 再定位 (默认过期时间3s)
   *      先使用Google定位, 定位失败后使用Tencent定位, 定位失败使用GPS定位, 最后再使用网络定位
   *  2. 连续定位: 直接使用Google定位, 然后使用Tencent定位, 然后使用GPS定位, 最后再使用网络定位
   *
   *  @param watch 是否是连续定位, 默认 false
   *  @param rate 连续定位的频率 / 单词定位的过期时间, 默认 1000ms
   *  @param timeout 发起定位请求的超时时间, 默认 800ms
   */
  fun locate(watch: Boolean?, rate: Long?, timeout: Long?, successCallback: SuccessCallback, failedCallback: FailedCallback ) {
    this.watch = watch ?: this.watch
    this.rate = rate?: this.rate
    this.timeout = timeout?: this.timeout
    this.successCallback = successCallback
    this.failedCallback = failedCallback
    clearUpdates()

    // 检查缓存地址是否有效
    var currentLocation: Location? = null;

    currentLocation = getLastLocation()
    if (currentLocation != null) {
      successCallback(currentLocation) // 确保在主线程执行
      if (!this.watch) {
        clearUpdates()
        return
      }
    }
    // 监控新地址
    if (
      watchCurrentLocationFromGoogle()
      || watchCurrentLocationFromTencent()
      || watchCurrentLocationFromGPS()
      || watchCurrentLocationAnyway()
    ) {
      // 如果不是监控模式, 立即取消监听
      if (!this.watch) {
//        clearUpdates()
      }
    } else {
        failedCallback("定位失败, 你需要开启定位服务或者连接网络!") // 确保在主线程执行
      clearUpdates()
    }
    return
  }


  override fun onLocationChanged(location: Location) {
    try {
      successCallback(location)
    } catch (e: Exception) {
      Log.e("location exception", e.message.toString())
    }
    Log.d("location changed", location.toString())
    if (!this.watch) {
      locationManager.removeUpdates(this);
    }
  }
}