// import { z } from "zod";
import { Query_create_delivery_task } from "@/api/task/query";
import type { BodyForCreateDeliveryTask } from "@/api/task/create.query.d";
import z from "zod";

export const FormSchemaForCreateDeliveryTask = z.object({
  shipping_method: z.string(),
  address_starting: z.object({
    poiname: z.any(),
    cityname: z.any(),
    poiaddress: z.any(),
    latlng: z.object({
      lat: z.any(),
      lng: z.any(),
    }),
  }),
  address_receive: z.object({
    poiname: z.any(),
    cityname: z.any(),
    poiaddress: z.any(),
    latlng: z.object({
      lat: z.any(),
      lng: z.any(),
    }),
  }),
  floor_room_number: z.string().nonempty(),
  name_receive: z.string().nonempty(),
  phone_number_receive: z.string().nonempty(),
  need_save: z.boolean(),
});

const normalize_request_body_for__create_delivery_task = (
  s: z.infer<typeof FormSchemaForCreateDeliveryTask>,
): BodyForCreateDeliveryTask => {
  // 查询当前用户的信息
  const normalize_result: BodyForCreateDeliveryTask = {
    need_save: s.need_save,
    shipping_method: s.shipping_method,
    user_send: {
      location: {
        cityname: s.address_starting.cityname,
        latlng_lat: s.address_starting.latlng.lat,
        latlng_lng: s.address_starting.latlng.lng,
        poiaddress: s.address_starting.poiaddress,
        poiname: s.address_starting.poiname,
      },
      floor_room_number: s.address_starting.poiname,
      name: "",
      phone_number: "",
    },
    user_receive: {
      location: {
        cityname: s.address_receive.cityname,
        latlng_lat: s.address_receive.latlng.lat,
        latlng_lng: s.address_receive.latlng.lng,
        poiaddress: s.address_receive.poiaddress,
        poiname: s.address_receive.poiname,
      },
      floor_room_number: s.floor_room_number,
      name: s.name_receive,
      phone_number: s.phone_number_receive,
    },
  };
  console.debug("normalize update task request body success!");
  return normalize_result;
};

export const CreateDataForTask = {
  symbol: Query_create_delivery_task,
  path: "/api/task/{id}",
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: {} as BodyForCreateDeliveryTask,
  zodSchema: FormSchemaForCreateDeliveryTask,
  normalize_req: normalize_request_body_for__create_delivery_task,
} as const;
