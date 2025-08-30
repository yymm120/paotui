import type { DeliveryTask, Notification } from "@/types";

export interface DeliveryAppState {
  // 激活的选项卡 "新任务" | "取货" | "配送"
  active_tab: "new-tasks" | "pickup" | "delivery";
  // 工作状态
  working_status: "on" | "off";
  // 排序方式
  filter_type: "comprehensive" | "distance" | "earnings" | "time";
  // 订单列表
  orders: DeliveryTask[];
  // 接受的订单
  accepted_orders: string[];
  // 通知列表
  notifications: Notification[];
}
