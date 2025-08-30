// Utility Types for Enterprise Compliance

// import type { DeliveryOrder } from './delivery';

// Utility function return types
export type FilterFunction<T> = (items: T[], criteria: string) => T[];
export type SortFunction<T> = (items: T[]) => T[];
export type ValidatorFunction<T> = (value: T) => boolean;
export type TransformFunction<T, U> = (input: T) => U;

// Delivery utility types
// export type DeliveryOrderStatus = DeliveryOrder['status'];
// export type DeliveryOrderPriority = DeliveryOrder['priority'];
export type DeliveryFilterType =
  | "comprehensive"
  | "distance"
  | "earnings"
  | "time";
export type DeliveryTabType = "new-tasks" | "pickup" | "delivery";

// Location and distance types
export interface Coordinates {
  latitude: number;
  longitude: number;
}

export interface DistanceCalculation {
  from: Coordinates;
  to: Coordinates;
  distance: number;
  unit: "meters" | "kilometers";
}

// Time and duration types
export interface TimeRange {
  start: string; // HH:MM format
  end: string; // HH:MM format
}

export interface Duration {
  value: number;
  unit: "minutes" | "hours" | "days";
}

// Validation result types
export interface ValidationResult {
  isValid: boolean;
  errors: string[];
  warnings?: string[];
}

export interface PhoneValidationResult extends ValidationResult {
  formattedNumber?: string;
  countryCode?: string;
}

export interface AddressValidationResult extends ValidationResult {
  formattedAddress?: string;
  coordinates?: Coordinates;
}

// Formatting types
export interface FormatOptions {
  locale?: string;
  currency?: string;
  precision?: number;
}

export interface DistanceFormatOptions extends FormatOptions {
  unit?: "auto" | "meters" | "kilometers";
  showUnit?: boolean;
}

export interface TimeFormatOptions extends FormatOptions {
  format?: "12h" | "24h";
  showSeconds?: boolean;
}

// Performance and optimization types
export interface DebounceOptions {
  wait: number;
  leading?: boolean;
  trailing?: boolean;
}

export interface ThrottleOptions {
  limit: number;
  leading?: boolean;
  trailing?: boolean;
}

// Storage and persistence types
export interface StorageItem<T> {
  value: T;
  timestamp: number;
  expiresAt?: number;
}

export interface ExportData {
  timestamp: string;
  version: string;
  data: Record<string, unknown>;
}

// Network and connectivity types
export type NetworkType = "2g" | "3g" | "4g" | "slow-2g" | "wifi" | "unknown";
export type ConnectionStatus = "online" | "offline" | "poor";

export interface NetworkStatus {
  isOnline: boolean;
  networkType: NetworkType;
  effectiveType?: string;
  downlink?: number;
  rtt?: number;
}

// Error handling types
export interface AppError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
  timestamp: Date;
  stack?: string;
}

export type ErrorHandler = (error: AppError) => void;
export type AsyncErrorHandler = (error: AppError) => Promise<void>;

// Notification types
export type NotificationType = "info" | "success" | "warning" | "error";
export type NotificationPriority = "low" | "normal" | "high" | "urgent";

export interface NotificationOptions {
  title: string;
  body: string;
  type?: NotificationType;
  priority?: NotificationPriority;
  persistent?: boolean;
  actions?: NotificationAction[];
}

export interface NotificationAction {
  id: string;
  title: string;
  icon?: string;
}

// File and media types
export interface FileInfo {
  name: string;
  size: number;
  type: string;
  lastModified: number;
}

export interface PhotoMetadata {
  width: number;
  height: number;
  fileSize: number;
  format: string;
  timestamp: string;
  location?: Coordinates;
}

// Async db types
export type AsyncOperation<T> = Promise<T>;
export type AsyncCallback<T> = (result: T) => Promise<void>;
export type ErrorCallback = (error: Error) => void;

// State management types
export type StateUpdater<T> = (prevState: T) => T;
export type AsyncStateUpdater<T> = (prevState: T) => Promise<T>;

// Generic utility types
export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;
export type RequiredFields<T, K extends keyof T> = T & Pick<Required<T>, K>;
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

// Function composition types
export type UnaryFunction<T, U> = (arg: T) => U;
export type BinaryFunction<T, U, V> = (arg1: T, arg2: U) => V;
export type Predicate<T> = (value: T) => boolean;
export type Comparator<T> = (a: T, b: T) => number;
