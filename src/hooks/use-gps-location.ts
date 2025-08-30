import {
  checkPermissions,
  requestPermissions,
  getCurrentPosition,
  watchPosition,
} from "@tauri-apps/plugin-geolocation";
import { Channel, isTauri } from "@tauri-apps/api/core";
import { toast } from "sonner";

export const useGpsLocation = () => {
  console.log(isTauri());

  // if (isTauri()) {
  //   checkPermissions().then((permissions) => {
  //     // let channel = new Channel()
  //     // channel.onmessage
  //     // console.log("location permissions is: ", JSON.stringify((permissions.location)));
  //     // toast(`${JSON.stringify((permissions))}`)
  //   });

  // requestPermissions()
  // getCurrentPosition()
  // watchPosition()
  // }
};
