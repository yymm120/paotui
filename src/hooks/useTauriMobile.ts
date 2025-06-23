import { useState, useEffect, useCallback } from "react";

// Conditional imports for Tauri (only when available)
let invoke: any = null;
let isPermissionGranted: any = null;
let requestPermission: any = null;
let sendNotification: any = null;
let getCurrentPosition: any = null;
let watchPosition: any = null;
let takePicture: any = null;
let writeTextFile: any = null;

// Check if running in Tauri environment
const isTauriEnvironment =
  typeof window !== "undefined" && (window as any).__TAURI__ !== undefined;

if (isTauriEnvironment) {
  try {
    // Only import Tauri APIs if in Tauri environment
    import("@tauri-apps/api/core").then((module) => {
      invoke = module.invoke;
    });
    import("@tauri-apps/plugin-notification").then((module) => {
      isPermissionGranted = module.isPermissionGranted;
      requestPermission = module.requestPermission;
      sendNotification = module.sendNotification;
    });
    import("@tauri-apps/plugin-geolocation").then((module) => {
      getCurrentPosition = module.getCurrentPosition;
      watchPosition = module.watchPosition;
    });
    import("@tauri-apps/plugin-camera").then((module) => {
      takePicture = module.takePicture;
    });
    import("@tauri-apps/plugin-fs").then((module) => {
      writeTextFile = module.writeTextFile;
    });
  } catch (error) {
    console.warn("Tauri APIs not available:", error);
  }
}

export interface TauriDeliveryOrder {
  id: string;
  delivery_time: string;
  rating: string;
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
  const [currentLocation, setCurrentLocation] = useState<LocationData | null>(
    null,
  );
  const [isWorking, setIsWorking] = useState(false);
  const [settings, setSettings] = useState<DeliverySettings>({
    auto_accept_orders: false,
    min_order_value: 10.0,
    max_delivery_distance: 5000.0,
    sound_notifications: true,
    working_hours_start: "08:00",
    working_hours_end: "22:00",
  });
  const [isLocationWatching, setIsLocationWatching] = useState(false);

  // Early return with safe defaults if not in Tauri environment
  if (!isTauriEnvironment) {
    return {
      orders: [],
      currentLocation: null,
      isWorking: false,
      settings,
      isLocationWatching: false,
      loadOrders: async () => {},
      acceptOrder: async () => {},
      updateOrderStatus: async () => {},
      toggleWorkStatus: async () => false,
      getCurrentLocationData: async () => null,
      startLocationTracking: async () => {},
      stopLocationTracking: () => {},
      takeDeliveryPhoto: async () => null,
      sendMobileNotification: async () => {},
      calculateDistance: async () => 0,
      loadSettings: async () => {},
      saveSettings: async () => {},
      exportData: async () => {},
    };
  }

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

  // Accept an order
  const acceptOrder = useCallback(
    async (orderId: string) => {
      if (!invoke) return;
      try {
        await invoke("accept_order", { orderId });
        await loadOrders(); // Refresh orders

        // Send notification
        await sendMobileNotification(
          "订单已接受",
          "您已成功接受新订单，请前往取货",
        );
      } catch (error) {
        console.error("Failed to accept order:", error);
      }
    },
    [loadOrders],
  );

  // Update order status
  const updateOrderStatus = useCallback(
    async (orderId: string, status: string) => {
      try {
        await invoke("update_order_status", { orderId, status });
        await loadOrders();
      } catch (error) {
        console.error("Failed to update order status:", error);
      }
    },
    [loadOrders],
  );

  // Toggle work status
  const toggleWorkStatus = useCallback(async () => {
    try {
      const newStatus = await invoke<boolean>("toggle_work_status");
      setIsWorking(newStatus);

      if (newStatus) {
        await sendMobileNotification("开始工作", "您已开始接收新订单");
        startLocationTracking();
      } else {
        await sendMobileNotification("结束工作", "您已停止接收新订单");
        stopLocationTracking();
      }
    } catch (error) {
      console.error("Failed to toggle work status:", error);
    }
  }, []);

  // Get current location
  const getCurrentLocationData = useCallback(async () => {
    try {
      const position = await getCurrentPosition();
      const locationData: LocationData = {
        latitude: position.coords.latitude,
        longitude: position.coords.longitude,
        accuracy: position.coords.accuracy,
        timestamp: new Date().toISOString(),
      };

      setCurrentLocation(locationData);
      await invoke("update_location", {
        latitude: locationData.latitude,
        longitude: locationData.longitude,
        accuracy: locationData.accuracy,
      });

      return locationData;
    } catch (error) {
      console.error("Failed to get location:", error);
      return null;
    }
  }, []);

  // Start location tracking
  const startLocationTracking = useCallback(async () => {
    if (isLocationWatching) return;

    try {
      setIsLocationWatching(true);

      // Watch position changes
      const unwatch = await watchPosition(
        { enableHighAccuracy: true, timeout: 10000, maximumAge: 60000 },
        (position) => {
          const locationData: LocationData = {
            latitude: position.coords.latitude,
            longitude: position.coords.longitude,
            accuracy: position.coords.accuracy,
            timestamp: new Date().toISOString(),
          };
          setCurrentLocation(locationData);

          // Update backend
          invoke("update_location", {
            latitude: locationData.latitude,
            longitude: locationData.longitude,
            accuracy: locationData.accuracy,
          });
        },
        (error) => {
          console.error("Location watch error:", error);
        },
      );

      // Store unwatch function for cleanup
      (window as any).locationUnwatch = unwatch;
    } catch (error) {
      console.error("Failed to start location tracking:", error);
      setIsLocationWatching(false);
    }
  }, [isLocationWatching]);

  // Stop location tracking
  const stopLocationTracking = useCallback(() => {
    if ((window as any).locationUnwatch) {
      (window as any).locationUnwatch();
      (window as any).locationUnwatch = null;
    }
    setIsLocationWatching(false);
  }, []);

  // Take delivery photo
  const takeDeliveryPhoto = useCallback(async (orderId: string) => {
    try {
      const photo = await takePicture({
        quality: 80,
        allowEdit: true,
        sourceType: "camera",
      });

      if (photo) {
        const savedPath = await invoke<string>("save_delivery_photo", {
          orderId,
          photoPath: photo.webPath || photo.path,
        });

        await sendMobileNotification(
          "送达照片已保存",
          "订单送达照片已成功保存",
        );

        return savedPath;
      }
    } catch (error) {
      console.error("Failed to take delivery photo:", error);
    }
    return null;
  }, []);

  // Send mobile notification
  const sendMobileNotification = useCallback(
    async (title: string, body: string) => {
      if (!isPermissionGranted || !requestPermission || !sendNotification) {
        console.log(`📱 Mobile Notification: ${title} - ${body}`);
        return;
      }

      try {
        let permissionGranted = await isPermissionGranted();

        if (!permissionGranted) {
          const permission = await requestPermission();
          permissionGranted = permission === "granted";
        }

        if (permissionGranted) {
          await sendNotification({
            title,
            body,
            icon: "icons/128x128.png",
          });
        }
      } catch (error) {
        console.error("Failed to send notification:", error);
        console.log(`📱 Fallback Notification: ${title} - ${body}`);
      }
    },
    [],
  );

  // Calculate route distance
  const calculateDistance = useCallback(
    async (fromLat: number, fromLng: number, toLat: number, toLng: number) => {
      try {
        const distance = await invoke<number>("calculate_route_distance", {
          fromLat,
          fromLng,
          toLat,
          toLng,
        });
        return distance;
      } catch (error) {
        console.error("Failed to calculate distance:", error);
        return 0;
      }
    },
    [],
  );

  // Load settings
  const loadSettings = useCallback(async () => {
    try {
      const settingsData = await invoke<DeliverySettings>("get_settings");
      setSettings(settingsData);
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  }, []);

  // Save settings
  const saveSettings = useCallback(async (newSettings: DeliverySettings) => {
    try {
      await invoke("update_settings", { newSettings });
      setSettings(newSettings);
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  }, []);

  // Export/Import data
  const exportData = useCallback(async () => {
    try {
      const data = {
        orders,
        settings,
        exportedAt: new Date().toISOString(),
      };

      const jsonData = JSON.stringify(data, null, 2);
      const fileName = `paotui_backup_${new Date().toISOString().split("T")[0]}.json`;

      await writeTextFile(fileName, jsonData, {
        baseDir: BaseDirectory.Download,
      });

      await sendMobileNotification(
        "数据导出完成",
        `备份文件已保存到下载目录: ${fileName}`,
      );
    } catch (error) {
      console.error("Failed to export data:", error);
    }
  }, [orders, settings]);

  // Initialize data on mount
  useEffect(() => {
    loadOrders();
    loadSettings();

    // Get initial work status
    invoke<boolean>("get_work_status").then(setIsWorking);

    // Get initial location
    getCurrentLocationData();
  }, [loadOrders, loadSettings, getCurrentLocationData]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      stopLocationTracking();
    };
  }, [stopLocationTracking]);

  return {
    // State
    orders,
    currentLocation,
    isWorking,
    settings,
    isLocationWatching,

    // Actions
    loadOrders,
    acceptOrder,
    updateOrderStatus,
    toggleWorkStatus,
    getCurrentLocationData,
    startLocationTracking,
    stopLocationTracking,
    takeDeliveryPhoto,
    sendMobileNotification,
    calculateDistance,
    loadSettings,
    saveSettings,
    exportData,
  };
}
