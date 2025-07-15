import {useState, useCallback, useEffect} from "react";
import { invoke} from "@tauri-apps/api/core";
// import {isPermissionGranted, requestPermission, sendNotification} from "@tauri-apps/plugin-notification"
// import { writeTextFile, BaseDirectory } from "@tauri-apps/plugin-fs"
// import {type Position, watchPosition} from "@tauri-apps/plugin-geolocation";
// import {takePicture }  from "@tauri-apps/plugin-camera"


// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
// const invoke = window.__TAURI__.core.invoke;

// Invoke the command
export interface TauriDeliveryOrder {
  id: string;
  delivery_time: string;
  estimatedEarnings: string;
  from_store: string;
  from_address: string;
  to_address: string;
  from_distance: string;
  to_distance: string;
  tag?: string;
  items: string;
  notes?: string;
  button_text: string;
  status: "New" | "Pickup" | "Delivery" | "Completed";
  priority: "High" | "Medium" | "Low";
  estimated_earnings: number;
  order_time: string;
  created_at: string;
}

export interface LocationData {
  latitude: number;
  longitude: number;
  accuracy: number;
  timestamp: string;
}

export interface DeliverySettings {
  auto_accept_orders: boolean;
  min_order_value: number;
  max_delivery_distance: number;
  sound_notifications: boolean;
  working_hours_start: string;
  working_hours_end: string;
}

export function useTauriMobile() {
  const [orders, setOrders] = useState<TauriDeliveryOrder[]>([]);
  const [currentLocation, ] = useState<LocationData | null>(
    null,
  );
  const [isWorking, ] = useState(false);
  const [settings, ] = useState<DeliverySettings>({
    auto_accept_orders: false,
    min_order_value: 10.0,
    max_delivery_distance: 5000.0,
    sound_notifications: true,
    working_hours_start: "08:00",
    working_hours_end: "22:00",
  });

  // Load orders from Tauri backend
  const loadOrders = useCallback(async () => {
    if (!invoke) return;
    try {
      const ordersData = await invoke<TauriDeliveryOrder[]>("get_orders");
      setOrders(ordersData);
    } catch (error) {
      console.error("Failed to load orders:", error);
    }
  }, []);

  useEffect(() => {
      invoke('auth_init').then((message: string | unknown) => console.log(message));
  }, [])

  return {
    // State
    orders,
    currentLocation,
    isWorking,
    settings,
    // isLocationWatching,

    // Actions
    loadOrders,
    // acceptOrder,
    // updateOrderStatus,
    // toggleWorkStatus,
    // getCurrentLocationData,
    // startLocationTracking,
    // stopLocationTracking,
    // takeDeliveryPhoto,
    // sendMobileNotification,
    // calculateDistance,
    // loadSettings,
    // saveSettings,
    // exportData,
  };
}
