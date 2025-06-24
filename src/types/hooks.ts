// Hook Types and Return Types for Enterprise Compliance

import type { DeliveryOrder, DeliveryAppState, Notification } from './delivery';
import type { TauriDeliveryOrder, LocationData, DeliverySettings } from './tauri';

// useDeliveryApp Hook Return Type
export interface UseDeliveryAppReturn {
  // State
  activeTab: DeliveryAppState['activeTab'];
  isWorking: boolean;
  userStatus: string;
  orders: DeliveryOrder[];
  allOrders: DeliveryOrder[];
  notifications: Notification[];
  
  // Computed values
  currentFilterLabel: string;
  unreadNotificationsCount: number;
  ordersCounts: {
    newTasks: number;
    pickup: number;
    delivery: number;
  };
  
  // Actions
  handleTabChange: (tab: string) => void;
  handleToggleWork: () => void;
  handleAcceptOrder: (orderId: string) => void;
  handleFilterChange: () => void;
  handleMenuClick: () => void;
  handleStatusClick: () => void;
  handleNotificationClick: () => void;
  handleSettingsClick: () => void;
  addNewOrder: (order: Omit<DeliveryOrder, 'id'>) => void;
}

// useTauriMobile Hook Return Type
export interface UseTauriMobileReturn {
  // State
  orders: TauriDeliveryOrder[];
  currentLocation: LocationData | null;
  isWorking: boolean;
  settings: DeliverySettings;
  isLocationWatching: boolean;
  
  // Actions
  loadOrders: () => Promise<void>;
  acceptOrder: (orderId: string) => Promise<void>;
  updateOrderStatus: (orderId: string, status: string) => Promise<void>;
  toggleWorkStatus: () => Promise<boolean>;
  getCurrentLocationData: () => Promise<LocationData | null>;
  startLocationTracking: () => Promise<void>;
  stopLocationTracking: () => void;
  takeDeliveryPhoto: (orderId: string) => Promise<string | null>;
  sendMobileNotification: (title: string, body: string) => Promise<void>;
  calculateDistance: (fromLat: number, fromLng: number, toLat: number, toLng: number) => Promise<number>;
  loadSettings: () => Promise<void>;
  saveSettings: (settings: DeliverySettings) => Promise<void>;
  exportData: () => Promise<void>;
}

// Custom Hook Options
export interface UseLocationOptions {
  enableHighAccuracy?: boolean;
  timeout?: number;
  maximumAge?: number;
  watchPosition?: boolean;
}

export interface UseNotificationOptions {
  autoRequestPermission?: boolean;
  defaultIcon?: string;
  sound?: boolean;
}

export interface UseStorageOptions<T> {
  key: string;
  defaultValue: T;
  serializer?: {
    serialize: (value: T) => string;
    deserialize: (value: string) => T;
  };
}

// Event Handler Types
export type TabChangeHandler = (tab: string) => void;
export type OrderAcceptHandler = (orderId: string) => void;
export type FilterChangeHandler = () => void;
export type WorkToggleHandler = () => void;
export type NotificationHandler = () => void;
export type MenuHandler = () => void;
export type SettingsHandler = () => void;
export type PhotoTakenHandler = (path: string) => void;
export type LocationUpdateHandler = (latitude: number, longitude: number) => void;

// Async Event Handler Types
export type AsyncOrderAcceptHandler = (orderId: string) => Promise<void>;
export type AsyncLocationHandler = () => Promise<LocationData | null>;
export type AsyncNotificationHandler = (title: string, body: string) => Promise<void>;
export type AsyncPhotoHandler = (orderId: string) => Promise<string | null>;

// State Update Types
export type StateUpdater<T> = (prevState: T) => T;
export type AsyncStateUpdater<T> = (prevState: T) => Promise<T>;

// Hook Configuration Types
export interface HookConfig {
  enableLogging?: boolean;
  enableAnalytics?: boolean;
  errorBoundary?: boolean;
}

export interface DeliveryAppConfig extends HookConfig {
  autoAcceptOrders?: boolean;
  simulateNewOrders?: boolean;
  orderSimulationInterval?: number;
}

export interface TauriMobileConfig extends HookConfig {
  enableLocationTracking?: boolean;
  enableNotifications?: boolean;
  enableCamera?: boolean;
  locationUpdateInterval?: number;
}