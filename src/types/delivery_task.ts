export interface TaskLocation {
  poiname: string;
  cityname: string;
  poiaddress: string;
  latlng_lat: number;
  latlng_lng: number;
}
export interface DeliveryTask {
  id: string;
  // 跑腿任务的下发时间
  time_order: Date | undefined;

  // 跑腿的送达时间
  time_arrived: Date | undefined;
  // 顾客期望的送达时间
  time_expected_arrived: Date | undefined;
  // 跑腿的剩余时间
  time_remaining: number;
  // 跑腿的接单时间
  time_pickup?: Date;

  // 店铺地址
  address_send: TaskLocation;
  // 当前地址
  address_current: TaskLocation;
  // 顾客地址
  address_receive: TaskLocation;

  // 当前地址到店铺的距离
  distance_current_to_store: number;
  // 店铺到顾客的距离
  distance_store_to_customer: number;
  // 当前地址到顾客的距离
  distance_current_to_customer: number;

  telephone_send: string;
  username_send: string;
  telephone_receive: string;
  username_receive: string;

  // 标记
  tag?: string;
  // 货品清单
  items: string;
  // 备注
  notes?: string;
  // 运送状态
  status: "new" | "pickup" | "delivery" | "completed";
  // 运送优先级
  priority: "high" | "medium" | "low" | "default";
  // 预计收入
  estimated_in_come: String;
}
