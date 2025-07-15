import { useState, useCallback, useEffect } from "react";
import type {
   DeliveryAppState, DeliveryOrder,
} from "@/types";
import {mockDeliveryOrders, mockNotifications } from "../data/mockDeliveryOrders";
import {
  filterOrdersByStatus,
  sortOrdersByPriority,
  sortOrdersByDistance,
  sortOrdersByEarnings,
  sortOrdersByTime,
  getTabOrdersCount,
} from "../utils/delivery";
import WebSocket from '@tauri-apps/plugin-websocket';

// const mock = true;

const initialState: DeliveryAppState = {
  active_tab: "new-tasks",
  working_status: "off",
  filter_type: "comprehensive",
  orders: mockDeliveryOrders,
  accepted_orders: [],
  notifications: mockNotifications,
};

export function useDeliveryApp() {
  const [state, setState] = useState<DeliveryAppState>(initialState);



  // Get filtered and sorted orders based on current tab and filter
  const getCurrentOrders = useCallback((): DeliveryOrder[] => {
    let orders: DeliveryOrder[] = [];

    // Filter by tab
    switch (state.active_tab) {
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
    switch (state.filter_type) {
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
  }, [state.orders, state.active_tab, state.filter_type]);

  // Change active tab
  const handleTabChange = useCallback((tab: string) => {
    // console.log("tab change", tab)
    setState((prev) => {
      // console.log("prev", prev)
      return {
      ...prev, active_tab: tab as typeof prev.active_tab,
    }});
  }, []);

  // Toggle work status
  const handleToggleWork = useCallback(() => {
    // console.log("abcdefg")
    // TODO: Api - switch working status
    // 1. 查询状态
    // 2. 设置相反状态
    setState((prev) => {
      // console.log("prev", prev)
      const is_working = prev.working_status === "on";
      const working_status_switched = is_working ? "off" : "on";
      return {
      ...prev, working_status: working_status_switched,
    }});
  }, []);

  // Accept an order
  const handleAcceptOrder = useCallback((orderId: string) => {
    console.log("accepting order", orderId);
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
        acceptedOrders: [...prev.accepted_orders, orderId],
      };
    });

    // Show success feedback
    console.log(`Order ${orderId} accepted successfully!`);
  }, []);

  // Change filter type
  const handleFilterChange = useCallback(() => {
    setState((prev) => {
      const filters: (typeof prev.filter_type)[] = [
        "comprehensive",
        "distance",
        "earnings",
        "time",
      ];
      const currentIndex = filters.indexOf(prev.filter_type);
      const nextIndex = (currentIndex + 1) % filters.length;


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
    return filterLabels[state.filter_type];
  }, [state.filter_type]);

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
    if (state.working_status !== "on") return;

    const interval = setInterval(() => {
      // Randomly add a new order (10% chance every 30 seconds)
      if (Math.random() < 0.1) {
        const newOrderTemplate =
          mockDeliveryOrders[Math.floor(Math.random() * mockDeliveryOrders.length)];
        addNewOrder({
          ...newOrderTemplate,
          time_order: new Date(),
        });
      }
    }, 30000); // Check every 30 seconds

    return () => clearInterval(interval);
  }, [state.working_status, addNewOrder]);

  // console.log("status", state);

  return {
    // State
    activeTab: state.active_tab,
    isWorking: state.working_status === "on",
    // userStatus: state.userStatus,
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
