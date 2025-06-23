import type { TauriDeliveryOrder, LocationData } from "../hooks/useTauriMobile";

// Mobile platform detection
export function isMobile(): boolean {
  return (
    typeof window !== "undefined" && (window as any).__TAURI__ !== undefined
  );
}

export function isAndroid(): boolean {
  return isMobile() && navigator.userAgent.includes("Android");
}

// Distance calculations
export function formatDistance(meters: number): string {
  if (meters >= 1000) {
    return `${(meters / 1000).toFixed(1)}km`;
  }
  return `${Math.round(meters)}m`;
}

export function calculateTotalDistance(order: TauriDeliveryOrder): number {
  return parseInt(order.from_distance) + parseInt(order.to_distance);
}

// Time utilities
export function formatDeliveryTime(timeString: string): string {
  // Extract time from strings like "30分钟内(19:45前)送达"
  const timeMatch = timeString.match(/(\d+)分钟内/);
  if (timeMatch) {
    const minutes = parseInt(timeMatch[1]);
    return `${minutes}分钟内`;
  }
  return timeString;
}

export function getTimeUntilDelivery(deliveryTime: string): number {
  const timeMatch = deliveryTime.match(/(\d+)分钟内/);
  return timeMatch ? parseInt(timeMatch[1]) : 30;
}

export function isWithinWorkingHours(
  startTime: string = "08:00",
  endTime: string = "22:00",
): boolean {
  const now = new Date();
  const currentTime = now.getHours() * 60 + now.getMinutes();

  const [startHour, startMin] = startTime.split(":").map(Number);
  const [endHour, endMin] = endTime.split(":").map(Number);

  const start = startHour * 60 + startMin;
  const end = endHour * 60 + endMin;

  return currentTime >= start && currentTime <= end;
}

// Location utilities
export function calculateDistanceBetweenPoints(
  lat1: number,
  lng1: number,
  lat2: number,
  lng2: number,
): number {
  const R = 6371000; // Earth's radius in meters
  const φ1 = (lat1 * Math.PI) / 180;
  const φ2 = (lat2 * Math.PI) / 180;
  const Δφ = ((lat2 - lat1) * Math.PI) / 180;
  const Δλ = ((lng2 - lng1) * Math.PI) / 180;

  const a =
    Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
    Math.cos(φ1) * Math.cos(φ2) * Math.sin(Δλ / 2) * Math.sin(Δλ / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

  return R * c;
}

export function isLocationAccurate(location: LocationData): boolean {
  return location.accuracy <= 50; // Within 50 meters
}

export function formatCoordinates(lat: number, lng: number): string {
  return `${lat.toFixed(6)}, ${lng.toFixed(6)}`;
}

// Order utilities
export function getOrderStatusIcon(
  status: TauriDeliveryOrder["status"],
): string {
  const icons = {
    New: "📦",
    Pickup: "🚶",
    Delivery: "🚗",
    Completed: "✅",
  };
  return icons[status] || "📦";
}

export function getOrderPriorityColor(
  priority: TauriDeliveryOrder["priority"],
): string {
  const colors = {
    High: "text-red-600",
    Medium: "text-yellow-600",
    Low: "text-green-600",
  };
  return colors[priority] || "text-gray-600";
}

export function sortOrdersByDistance(
  orders: TauriDeliveryOrder[],
): TauriDeliveryOrder[] {
  return [...orders].sort((a, b) => {
    const distanceA = calculateTotalDistance(a);
    const distanceB = calculateTotalDistance(b);
    return distanceA - distanceB;
  });
}

export function sortOrdersByEarnings(
  orders: TauriDeliveryOrder[],
): TauriDeliveryOrder[] {
  return [...orders].sort(
    (a, b) => b.estimated_earnings - a.estimated_earnings,
  );
}

export function sortOrdersByPriority(
  orders: TauriDeliveryOrder[],
): TauriDeliveryOrder[] {
  const priorityOrder = { High: 3, Medium: 2, Low: 1 };
  return [...orders].sort(
    (a, b) => priorityOrder[b.priority] - priorityOrder[a.priority],
  );
}

export function filterOrdersByStatus(
  orders: TauriDeliveryOrder[],
  status: TauriDeliveryOrder["status"],
): TauriDeliveryOrder[] {
  return orders.filter((order) => order.status === status);
}

// Notification utilities
export function createOrderNotificationTitle(
  order: TauriDeliveryOrder,
): string {
  return `新订单：${order.from_store}`;
}

export function createOrderNotificationBody(order: TauriDeliveryOrder): string {
  const distance = formatDistance(calculateTotalDistance(order));
  return `配送距离：${distance} | 预计收益：¥${order.estimated_earnings}`;
}

// File and data utilities
export async function saveOrderData(
  orders: TauriDeliveryOrder[],
): Promise<void> {
  try {
    const data = {
      orders,
      timestamp: new Date().toISOString(),
      version: "1.0.0",
    };

    // Use localStorage as fallback when Tauri is not available
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("paotui_orders_backup", JSON.stringify(data));
    }

    console.log("Order data saved to localStorage");
  } catch (error) {
    console.error("Failed to save order data:", error);
  }
}

export async function loadOrderData(): Promise<TauriDeliveryOrder[] | null> {
  try {
    // Use localStorage as fallback when Tauri is not available
    if (typeof localStorage !== "undefined") {
      const data = localStorage.getItem("paotui_orders_backup");
      if (data) {
        const parsed = JSON.parse(data);
        return parsed.orders || null;
      }
    }

    return null;
  } catch (error) {
    console.error("Failed to load order data:", error);
    return null;
  }
}

// Performance utilities
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number,
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout | null = null;

  return (...args: Parameters<T>) => {
    if (timeout) {
      clearTimeout(timeout);
    }

    timeout = setTimeout(() => {
      func(...args);
    }, wait);
  };
}

export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number,
): (...args: Parameters<T>) => void {
  let inThrottle: boolean = false;

  return (...args: Parameters<T>) => {
    if (!inThrottle) {
      func(...args);
      inThrottle = true;
      setTimeout(() => (inThrottle = false), limit);
    }
  };
}

// Validation utilities
export function validatePhoneNumber(phone: string): boolean {
  const phoneRegex = /^1[3-9]\d{9}$/;
  return phoneRegex.test(phone);
}

export function validateAddress(address: string): boolean {
  return address.length >= 5 && address.length <= 100;
}

export function validateOrderId(orderId: string): boolean {
  return /^[a-zA-Z0-9-_]{10,50}$/.test(orderId);
}

// App lifecycle utilities
export function handleAppPause(): void {
  console.log("App paused - saving state");
  // Save current state when app goes to background
}

export function handleAppResume(): void {
  console.log("App resumed - refreshing data");
  // Refresh data when app comes back to foreground
}

// Battery optimization
export function isBatteryOptimizationEnabled(): boolean {
  // Check if battery optimization might affect location tracking
  return navigator.getBattery !== undefined;
}

export async function getBatteryLevel(): Promise<number> {
  try {
    if ("getBattery" in navigator) {
      const battery = await (navigator as any).getBattery();
      return battery.level * 100;
    }
  } catch (error) {
    console.error("Failed to get battery level:", error);
  }
  return 100; // Default to full battery
}

// Network utilities
export function isOnline(): boolean {
  return navigator.onLine;
}

export function getNetworkType(): string {
  const connection =
    (navigator as any).connection ||
    (navigator as any).mozConnection ||
    (navigator as any).webkitConnection;
  return connection ? connection.effectiveType || "unknown" : "unknown";
}

// Emergency utilities
export function createEmergencyContact(): void {
  // This would open emergency contact options
  console.log("Emergency contact feature");
}

export function reportIssue(issue: string): void {
  // This would report technical issues
  console.log("Reporting issue:", issue);
}
