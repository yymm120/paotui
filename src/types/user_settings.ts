export interface UserSettings {
  // 自动接单模式
  auto_accept_orders: boolean;
  // 最低订单金额
  min_order_value: number;
  // 最大配送距离
  max_delivery_distance: number;
  // 工作时间
  working_hours: {
    start: string;
    end: string;
  };
  sound_notifications: boolean;
}
