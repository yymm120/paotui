import { type DeliveryOrder } from "@/types";

export function filterOrdersByStatus(
  orders: DeliveryOrder[],
  status: DeliveryOrder["status"],
): DeliveryOrder[] {
  return orders.filter((order) => order.status === status);
}

export function sortOrdersByPriority(orders: DeliveryOrder[]): DeliveryOrder[] {
  const priorityOrder = { high: 3, medium: 2, low: 1 };
  return [...orders].sort(
    (a, b) => priorityOrder[b.priority] - priorityOrder[a.priority],
  );
}

export function sortOrdersByDistance(orders: DeliveryOrder[]): DeliveryOrder[] {
  return [...orders].sort((a, b) => {
    const totalDistanceA = parseInt(a.distance_current_to_store) + parseInt(a.distance_store_to_customer);
    const totalDistanceB = parseInt(b.distance_current_to_store) + parseInt(b.distance_store_to_customer);
    return totalDistanceA - totalDistanceB;
  });
}

export function sortOrdersByEarnings(orders: DeliveryOrder[]): DeliveryOrder[] {
  return [...orders].sort((a, b) => b.estimatedEarnings - a.estimatedEarnings);
}

export function sortOrdersByTime(orders: DeliveryOrder[]): DeliveryOrder[] {
  return [...orders].sort((a, b) => {
    return a.time_order!.getTime() - b.time_order!.getTime();
  });
}

export function formatDistance(distance: string): string {
  const dist = parseInt(distance);
  if (dist >= 1000) {
    return `${(dist / 1000).toFixed(1)}km`;
  }
  return `${dist}m`;
}

export function formatEarnings(earnings: number): string {
  return `￥${earnings.toFixed(2)}`;
}

export function calculateTotalDistance(order: DeliveryOrder): number {
  return parseInt(order.distance_current_to_store) + parseInt(order.distance_store_to_customer);
}

export function getOrderStatusText(status: DeliveryOrder["status"]): string {
  const statusMap = {
    new: "新订单",
    pickup: "待取货",
    delivery: "配送中",
    completed: "已完成",
  };
  return statusMap[status];
}

export function getTabOrdersCount(
  orders: DeliveryOrder[],
  tab: string,
): number {
  switch (tab) {
    case "new-tasks":
      return filterOrdersByStatus(orders, "new").length;
    case "pickup":
      return filterOrdersByStatus(orders, "pickup").length;
    case "delivery":
      return filterOrdersByStatus(orders, "delivery").length;
    default:
      return 0;
  }
}

export function generateOrderId(): string {
  return Date.now().toString(36) + Math.random().toString(36).substr(2);
}

export function isWorkingHours(): boolean {
  const now = new Date();
  const hour = now.getHours();
  // Assume working hours are 8 AM to 10 PM
  return hour >= 8 && hour <= 22;
}

export function getTimeUntilDelivery(deliveryTime: string): number {
  // Extract time from delivery time string like "30分钟内(19:45前)送达"
  const timeMatch = deliveryTime.match(/(\d+)分钟内/);
  if (timeMatch) {
    return parseInt(timeMatch[1]);
  }
  return 30; // default 30 minutes
}
