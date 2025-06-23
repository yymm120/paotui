export interface DeliveryOrder {
  id: string;
  deliveryTime: string;
  rating: string;
  fromStore: string;
  fromAddress: string;
  toAddress: string;
  fromDistance: string;
  toDistance: string;
  tag?: string;
  items: string;
  notes?: string;
  buttonText: string;
  buttonColor?: string;
  status: "new" | "pickup" | "delivery" | "completed";
  priority: "high" | "medium" | "low";
  estimatedEarnings: number;
  orderTime: string;
}

export interface DeliveryAppState {
  activeTab: "new-tasks" | "pickup" | "delivery";
  isWorking: boolean;
  userStatus: string;
  filterType: "comprehensive" | "distance" | "earnings" | "time";
  orders: DeliveryOrder[];
  acceptedOrders: string[];
  notifications: Notification[];
}

export interface Notification {
  id: string;
  title: string;
  message: string;
  type: "info" | "success" | "warning" | "error";
  timestamp: Date;
  read: boolean;
}

export interface UserSettings {
  autoAcceptOrders: boolean;
  minOrderValue: number;
  maxDeliveryDistance: number;
  workingHours: {
    start: string;
    end: string;
  };
  soundNotifications: boolean;
}
