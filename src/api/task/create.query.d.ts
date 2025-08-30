export interface Location {
  poiname: string;
  cityname: string;
  poiaddress: string;
  latlng_lat: number;
  latlng_lng: number;
}
export interface BodyForCreateDeliveryTask {
  shipping_method: string;
  user_send: {
    name: string;
    phone_number: string;
    floor_room_number: string;
    location: Location;
  };
  user_receive: {
    name: string;
    phone_number: string;
    floor_room_number: string;
    location: Location;
  };
  need_save: boolean;
}
