import { useState, useCallback, useEffect } from "react";
import {
  DeliveryAppState,
  DeliveryOrder,
  Notification,
} from "../types/delivery";
import { mockOrders, mockNotifications } from "../data/mockOrders";
import {
  filterOrdersByStatus,
  sortOrdersByPriority,
  sortOrdersByDistance,
  sortOrdersByEarnings,
  sortOrdersByTime,
  getTabOrdersCount,
} from "../utils/delivery";

const initialState: DeliveryAppState = {
  activeTab: "new-tasks",
  isWorking: false,
  userStatus: "已收工",
  filterType: "comprehensive",
  orders: mockOrders,
  acceptedOrders: [],
  notifications: mockNotifications,
};

export function useDeliveryApp() {
  const [state, setState] = useState<DeliveryAppState>(initialState);

  // Get filtered and sorted orders based on current tab and filter
  const getCurrentOrders = useCallback((): DeliveryOrder[] => {
    let orders: DeliveryOrder[] = [];

    // Filter by tab
    switch (state.activeTab) {
      case "new-tasks":
        orders = filterOrdersByStatus(state.orders, "new");
        break;
      case "pickup":
        orders = filterOrdersByStatus(state.orders, "pickup");
        break;
      case "delivery":
        orders = filterOrdersByStatus(state.orders, "delivery");
        break;
      default:
        orders = state.orders;
    }

    // Apply sorting based on filter type
    switch (state.filterType) {
      case "comprehensive":
        return sortOrdersByPriority(orders);
      case "distance":
        return sortOrdersByDistance(orders);
      case "earnings":
        return sortOrdersByEarnings(orders);
      case "time":
        return sortOrdersByTime(orders);
      default:
        return orders;
    }
  }, [state.orders, state.activeTab, state.filterType]);

  // Change active tab
  const handleTabChange = useCallback((tab: string) => {
    setState((prev) => ({
      ...prev,
      activeTab: tab as typeof prev.activeTab,
    }));
  }, []);

  // Toggle work status
  const handleToggleWork = useCallback(() => {
    setState((prev) => ({
      ...prev,
      isWorking: !prev.isWorking,
      userStatus: prev.isWorking ? "已收工" : "工作中",
    }));
  }, []);

  // Accept an order
  const handleAcceptOrder = useCallback((orderId: string) => {
    setState((prev) => {
      const updatedOrders = prev.orders.map((order) => {
        if (order.id === orderId && order.status === "new") {
          return {
            ...order,
            status: "pickup" as const,
            buttonText: "前往取货",
          };
        }
        return order;
      });

      return {
        ...prev,
        orders: updatedOrders,
        acceptedOrders: [...prev.acceptedOrders, orderId],
      };
    });

    // Show success feedback
    console.log(`Order ${orderId} accepted successfully!`);
  }, []);

  // Change filter type
  const handleFilterChange = useCallback(() => {
    setState((prev) => {
      const filters: (typeof prev.filterType)[] = [
        "comprehensive",
        "distance",
        "earnings",
        "time",
      ];
      const currentIndex = filters.indexOf(prev.filterType);
      const nextIndex = (currentIndex + 1) % filters.length;

      const filterLabels = {
        comprehensive: "综合排序",
        distance: "距离优先",
        earnings: "收益优先",
        time: "时间优先",
      };

      return {
        ...prev,
        filterType: filters[nextIndex],
      };
    });
  }, []);

  // Get current filter label
  const getCurrentFilterLabel = useCallback((): string => {
    const filterLabels = {
      comprehensive: "综合排序",
      distance: "距离优先",
      earnings: "收益优先",
      time: "时间优先",
    };
    return filterLabels[state.filterType];
  }, [state.filterType]);

  // Handle menu click
  const handleMenuClick = useCallback(() => {
    console.log("Menu clicked - would open sidebar");
  }, []);

  // Handle status click
  const handleStatusClick = useCallback(() => {
    console.log("Status clicked - would open status selection");
  }, []);

  // Handle notification click
  const handleNotificationClick = useCallback(() => {
    console.log("Notifications clicked - would open notification panel");
    setState((prev) => ({
      ...prev,
      notifications: prev.notifications.map((n) => ({ ...n, read: true })),
    }));
  }, []);

  // Handle settings click
  const handleSettingsClick = useCallback(() => {
    console.log("Settings clicked - would open settings panel");
  }, []);

  // Get unread notifications count
  const getUnreadNotificationsCount = useCallback((): number => {
    return state.notifications.filter((n) => !n.read).length;
  }, [state.notifications]);

  // Get orders count for each tab
  const getOrdersCounts = useCallback(() => {
    return {
      newTasks: getTabOrdersCount(state.orders, "new-tasks"),
      pickup: getTabOrdersCount(state.orders, "pickup"),
      delivery: getTabOrdersCount(state.orders, "delivery"),
    };
  }, [state.orders]);

  // Add new order (for simulation)
  const addNewOrder = useCallback((order: Omit<DeliveryOrder, "id">) => {
    const newOrder: DeliveryOrder = {
      ...order,
      id: Date.now().toString(),
    };

    setState((prev) => ({
      ...prev,
      orders: [newOrder, ...prev.orders],
    }));
  }, []);

  // Simulate receiving new orders periodically when working
  useEffect(() => {
    if (!state.isWorking) return;

    const interval = setInterval(() => {
      // Randomly add a new order (10% chance every 30 seconds)
      if (Math.random() < 0.1) {
        const newOrderTemplate =
          mockOrders[Math.floor(Math.random() * mockOrders.length)];
        addNewOrder({
          ...newOrderTemplate,
          orderTime: new Date().toLocaleTimeString("zh-CN", {
            hour: "2-digit",
            minute: "2-digit",
          }),
        });
      }
    }, 30000); // Check every 30 seconds

    return () => clearInterval(interval);
  }, [state.isWorking, addNewOrder]);

  return {
    // State
    activeTab: state.activeTab,
    isWorking: state.isWorking,
    userStatus: state.userStatus,
    orders: getCurrentOrders(),
    allOrders: state.orders,
    notifications: state.notifications,

    // Computed values
    currentFilterLabel: getCurrentFilterLabel(),
    unreadNotificationsCount: getUnreadNotificationsCount(),
    ordersCounts: getOrdersCounts(),

    // Actions
    handleTabChange,
    handleToggleWork,
    handleAcceptOrder,
    handleFilterChange,
    handleMenuClick,
    handleStatusClick,
    handleNotificationClick,
    handleSettingsClick,
    addNewOrder,
  };
}
