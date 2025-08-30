import type { DeliveryTask } from "@/types";
import Decimal from "decimal.js";
import { Query_list_delivery_task } from "@/api/task/query";

export const RetrieveDataForListTask = {
  symbol: Query_list_delivery_task,
  path: "/api/task",
  method: "GET",
  normalize_res: normalize_response_body_for__delivery_task_list,
} as const;

export function normalize_response_body_for__delivery_task_list(
  data: any[],
): DeliveryTask[] {
  const result: DeliveryTask[] = data.map((before: any): DeliveryTask => {
    let after: DeliveryTask = {} as DeliveryTask;
    try {
      after = {
        address_current: {
          poiname: before?.address_current?.poiname,
          cityname: before?.address_current?.cityname,
          poiaddress: before?.address_current?.poiaddress,
          latlng_lat: before?.address_current?.latlng_lat,
          latlng_lng: before?.address_current?.latlng_lng,
        },
        address_receive: {
          poiname: before?.address_receive?.poiname,
          cityname: before?.address_receive?.cityname,
          poiaddress: before?.address_receive?.poiaddress,
          latlng_lat: before?.address_receive?.latlng_lat,
          latlng_lng: before?.address_receive?.latlng_lng,
        },
        address_send: {
          poiname: before?.address_send.poiname,
          cityname: before?.address_send.cityname,
          poiaddress: before?.address_send.poiaddress,
          latlng_lat: before?.address_send.latlng_lat,
          latlng_lng: before?.address_send.latlng_lng,
        },
        distance_current_to_customer: 0,
        distance_current_to_store: 0,
        distance_store_to_customer: 0,
        estimated_in_come: new Decimal(before?.estimated_in_come).toString(),
        id: before.id,
        items: before?.items,
        notes: before?.notes,
        priority: before?.priority,
        status: before?.status,
        tag: before?.tag,
        telephone_receive: before?.telephone_receive,
        telephone_send: before?.telephone_send,
        time_arrived: before?.time_arrived,
        time_expected_arrived: before?.time_expected_arrived,
        time_order: before?.time_order,
        time_pickup: before?.time_pickup,
        time_remaining: before?.time_remaining,
        username_receive: before?.username_receive,
        username_send: before?.username_send,
      };
    } catch (error) {
      console.log(error);
    }
    return after;
  });
  // console.log(result)
  return result;
}
