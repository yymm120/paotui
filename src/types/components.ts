// Component Props Type Definitions for Enterprise Compliance

// import type { DeliveryOrder } from './delivery';

// Header Component Props
import type { DeliveryTask } from "@/types/delivery_task.ts";

export interface DeliveryAppHeaderProps {
  onMenuClick: () => void;
  onStatusClick: () => void;
  onNotificationClick: () => void;
  status: string;
}

// Navigation Component Props
export interface NavigationTabsProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

// Filter Component Props
export interface FilterSectionProps {
  filterLabel: string;
  onFilterClick: () => void;
}

// Card List Component Props
export interface DeliveryCardListProps {
  orders: DeliveryTask[];
  onAcceptOrder: (orderId: string) => void;
}

// Individual Card Component Props
export interface DeliveryCardProps {
  order: DeliveryTask;
  onAcceptOrder: (orderId: string) => void;
}

// Footer Component Props
export interface BottomFooterProps {
  isWorking: boolean;
  onToggleWork: () => void;
  onSettingsClick: () => void;
}

// Layout Component Props
export interface DeliveryAppLayoutProps {
  // State props
  activeTab: string;
  isWorking: boolean;
  status: string;
  filterLabel: string;
  orders: DeliveryTask[];

  // Event handlers
  onMenuClick: () => void;
  onStatusClick: () => void;
  onNotificationClick: () => void;
  onTabChange: (tab: string) => void;
  onFilterClick: () => void;
  onAcceptOrder: (orderId: string) => void;
  onToggleWork: () => void;
  onSettingsClick: () => void;
}

// Demo Controls Props
export interface DemoControlsProps {
  isVisible: boolean;
  onToggleVisibility: () => void;
  onAddRandomOrder: () => void;
  onSimulateNotification: () => void;
  onResetData: () => void;
}

// Mobile Component Props
export interface CameraComponentProps {
  orderId: string;
  onPhotoTaken: (path: string) => void;
}

export interface LocationComponentProps {
  onLocationUpdate?: (latitude: number, longitude: number) => void;
}

export interface NotificationComponentProps {
  onNotificationSent?: () => void;
}

export interface SettingsComponentProps {
  onClose: () => void;
}

// Icon Component Props
export interface IconProps {
  className?: string;
  size?: number;
  color?: string;
}

export interface ChevronDownIconProps extends IconProps {}
export interface HamburgerIconProps extends IconProps {}
export interface NotificationIconProps extends IconProps {}
export interface StatusIconProps extends IconProps {
  status: "online" | "offline" | "busy";
}

// Button Component Props
export interface ButtonProps {
  variant?: "primary" | "secondary" | "danger" | "success";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
  loading?: boolean;
  className?: string;
  children: React.ReactNode;
  onClick?: () => void;
}

// Modal Component Props
export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  children: React.ReactNode;
  size?: "sm" | "md" | "lg" | "xl";
}

// Form Component Props
export interface FormInputProps {
  label: string;
  type?: "text" | "email" | "password" | "number" | "tel";
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  error?: string;
}

export interface FormSelectProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  options: Array<{ value: string; label: string }>;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  error?: string;
}

export interface FormCheckboxProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  error?: string;
}
