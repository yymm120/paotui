import type { DeliveryTask } from "@/types";
// import type {Location} from "@/api/task-delivery-create.ts";
import Decimal from "decimal.js";

// {
//   "distance_current_to_store": 0,
//   "distance_store_to_customer": 0,
//   "distance_current_to_customer": 0,
//   "telephone_send": "17815349593",
//   "username_send": "罗先生",
//   "telephone_receive": "15730243188",
//   "username_receive": "张先生"
// },
export const mockDeliveryOrders: DeliveryTask[] = [
  {
    id: "DO-1001",
    time_order: new Date(""),
    time_arrived: new Date(""),
    time_expected_arrived: new Date(""),
    time_remaining: 1,
    time_pickup: new Date(),
    address_send: {
      poiname: "合川规划展览馆",
      cityname: "重庆市",
      poiaddress: "重庆市合川区南津街街道希尔安大道225号",
      latlng_lat: 29.973673,
      latlng_lng: 106.276723,
    },
    address_current: {
      poiname: "合川规划展览馆",
      cityname: "重庆市",
      poiaddress: "重庆市合川区南津街街道希尔安大道225号",
      latlng_lat: 29.973673,
      latlng_lng: 106.276723,
    },
    address_receive: {
      poiname: "合川规划展览馆",
      cityname: "重庆市",
      poiaddress: "重庆市合川区南津街街道希尔安大道225号",
      latlng_lat: 29.973673,
      latlng_lng: 106.276723,
    },
    distance_current_to_store: 0,
    distance_store_to_customer: 0,
    distance_current_to_customer: 0,
    telephone_send: "string",
    username_send: "string",
    telephone_receive: "string",
    username_receive: "string",
    tag: "string",
    items: "string",
    notes: "string",
    status: "new",
    priority: "default",
    estimated_in_come: new Decimal(123.4567).toString(),
  },
];

// export default mockDeliveryOrders;

export const mockNotifications = [
  {
    id: "1",
    title: "新订单提醒",
    message: "您有3个新的配送订单等待接单",
    type: "info" as const,
    timestamp: new Date(),
    read: false,
  },
  {
    id: "2",
    title: "系统升级通知",
    message: "系统将于今晚23:00-01:00进行维护升级",
    type: "warning" as const,
    timestamp: new Date(Date.now() - 30 * 60 * 1000),
    read: false,
  },
  {
    id: "3",
    title: "收益到账",
    message: "今日配送收益￥126.50已到账",
    type: "success" as const,
    timestamp: new Date(Date.now() - 60 * 60 * 1000),
    read: true,
  },
];
