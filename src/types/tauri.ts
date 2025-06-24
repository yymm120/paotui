// Tauri API Type Definitions for Enterprise Compliance

export interface TauriWindow {
  __TAURI__: TauriAPI;
}

export interface TauriAPI {
  invoke: <T = unknown>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
  convertFileSrc: (filePath: string, protocol?: string) => string;
}

export interface TauriInvokeFunction {
  <T = unknown>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}

export interface TauriPermissionGranted {
  isPermissionGranted(): Promise<boolean>;
}

export interface TauriPermissionRequest {
  requestPermission(): Promise<'granted' | 'denied' | 'default'>;
}

export interface TauriNotificationOptions {
  title: string;
  body: string;
  icon?: string;
}

export interface TauriSendNotification {
  sendNotification(options: TauriNotificationOptions): Promise<void>;
}

export interface TauriPosition {
  coords: {
    latitude: number;
    longitude: number;
    accuracy: number;
    altitude?: number | undefined;
    altitudeAccuracy?: number | undefined;
    heading?: number | undefined;
    speed?: number | undefined;
  };
  timestamp: number;
}

export interface TauriPositionOptions {
  enableHighAccuracy?: boolean;
  timeout?: number;
  maximumAge?: number;
}

export interface TauriPositionError {
  code: number;
  message: string;
}

export interface TauriGeolocation {
  getCurrentPosition(): Promise<TauriPosition>;
  watchPosition(
    options: TauriPositionOptions,
    successCallback: (position: TauriPosition) => void,
    errorCallback: (error: TauriPositionError) => void
  ): Promise<() => void>;
}

export interface TauriPhotoOptions {
  quality?: number;
  allowEdit?: boolean;
  sourceType?: 'camera' | 'library';
}

export interface TauriPhoto {
  path?: string;
  webPath?: string;
  format?: string;
}

export interface TauriCamera {
  takePicture(options: TauriPhotoOptions): Promise<TauriPhoto>;
}

export interface TauriWriteOptions {
  baseDir?: string;
}

export interface TauriFileSystem {
  writeTextFile(fileName: string, content: string, options?: TauriWriteOptions): Promise<void>;
}

export interface TauriBaseDirectory {
  Download: string;
  Document: string;
  Data: string;
  Cache: string;
  Config: string;
}

// Tauri Mobile Specific Types
export interface TauriDeliveryOrder {
  id: string;
  delivery_time: string;
  rating: string;
  from_store: string;
  from_address: string;
  to_address: string;
  from_distance: string;
  to_distance: string;
  tag?: string | undefined;
  items: string;
  notes?: string | undefined;
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

// Battery API types
export interface BatteryManager {
  charging: boolean;
  chargingTime: number;
  dischargingTime: number;
  level: number;
  addEventListener(type: string, listener: EventListener): void;
  removeEventListener(type: string, listener: EventListener): void;
}

export interface ExtendedNavigator extends Navigator {
  getBattery?(): Promise<BatteryManager>;
  connection?: NetworkInformation;
  mozConnection?: NetworkInformation;
  webkitConnection?: NetworkInformation;
}

// Network API types
export interface NetworkInformation {
  effectiveType?: '2g' | '3g' | '4g' | 'slow-2g';
  type?: 'bluetooth' | 'cellular' | 'ethernet' | 'wifi' | 'wimax' | 'none' | 'other' | 'unknown';
  downlink?: number;
  downlinkMax?: number;
  rtt?: number;
  saveData?: boolean;
}

// Window extensions
declare global {
  interface Window extends TauriWindow {
    locationUnwatch?: (() => void) | null;
  }
}