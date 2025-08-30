import {
  type Dispatch,
  type RefObject,
  type SetStateAction,
  useEffect,
  useRef,
  useState,
} from "react";
import {
  getLocation,
  watchLocation,
  watchDirection,
  clearWatchLocation,
  clearWatchDirection,
} from "tauri-plugin-txmap-api";

export type LocationFeature = {
  location: [Location, Dispatch<SetStateAction<Location>>, RefObject<Location>];
  direction: [number, Dispatch<SetStateAction<number>>, RefObject<number>];
  watchTriggerOfLocation: [boolean, Dispatch<SetStateAction<boolean>>];
  watchTriggerOfDirection: [boolean, Dispatch<SetStateAction<boolean>>];
  channels: RefObject<{ [key: string]: { id: number } }>;
};

type Location = {
  /**
   * Creation time for these coordinates.
   */
  timestamp: number;
  /**
   * The GPD coordinates along with the accuracy of the data.
   */
  coords: Coordinates;
};
type Coordinates = {
  /**
   * Latitude in decimal degrees.
   */
  latitude: number;
  /**
   * Longitude in decimal degrees.
   */
  longitude: number;
  /**
   * Accuracy level of the latitude and longitude coordinates in meters.
   */
  accuracy: number;
  /**
   * Accuracy level of the altitude coordinate in meters, if available.
   * Available on all iOS versions and on Android 8 and above.
   */
  altitudeAccuracy: number | null;
  /**
   * The altitude the user is at, if available.
   */
  altitude: number | null;
  speed: number | null;
  /**
   * The heading the user is facing, if available.
   */
  heading: number | null;
};

export const useInitialLocation = (): LocationFeature => {
  // 经纬度
  const [currentLocation, setCurrentLocation] = useState<Location>({
    coords: { latitude: 30.005, longitude: 106.23 },
  } as Location);
  const locationRef = useRef<Location>({
    coords: { latitude: 30.005, longitude: 106.23 },
  } as Location);
  // 0: 默认指向北方
  const [currentDirection, setCurrentDirection] = useState<number>(0);
  const directionRef = useRef<number>(0);
  // trigger: 是否开启监听
  const [watchTriggerOfLocation, setWatchTriggerOfLocation] = useState(false);
  const [watchTriggerOfDirection, setWatchTriggerOfDirection] = useState(false);

  // 所有监听
  const channels = useRef<{ [key: string]: { id: number } }>({});

  const getCurrentLocation = async () => {
    getLocation({ timeout: 5000, age: 1000 * 60 * 60 * 12 }, (location) => {
      console.debug(`test location: ${JSON.stringify(location)}`);
      locationRef.current = location;
      setCurrentLocation(location);
    });
  };

  const startWatchLocation = async () => {
    if (
      channels.current &&
      typeof channels.current["location"] !== "undefined"
    ) {
      return;
    }
    const channelId = await watchLocation(
      { rate: 3000, timeout: 3000 },
      (location: Location) => {
        console.debug(`test location: ${JSON.stringify(location)}`);
        if (
          location &&
          typeof location.coords !== "undefined" &&
          typeof location.coords.latitude !== "undefined" &&
          typeof location.coords.longitude !== "undefined"
        ) {
          setCurrentLocation(location);
        }
      },
    );
    channels.current["location"] = { id: channelId };
  };

  const startWatchDirection = async () => {
    if (
      channels.current &&
      typeof channels.current["direction"] !== "undefined"
    ) {
      return;
    }
    const channelId = await watchDirection(
      { rate: 1000, timeout: 3000 },
      (direction: any) => {
        console.debug(`test direction: ${JSON.stringify(direction)}`);
        if (typeof direction === "number") {
          setCurrentDirection((prev) =>
            prev === direction ? prev : direction,
          );
        }
        channels.current["direction"] = { id: channelId };
      },
    );
  };

  const clearWatch = async (watchKey: string) => {
    switch (watchKey) {
      case "location": {
        const id = channels.current["location"]?.id;
        id && clearWatchLocation(id);
        break;
      }
      case "direction": {
        const id = channels.current["direction"]?.id;
        id && clearWatchDirection(id);
        break;
      }
      default: {
        console.error("错误, 没有找到watchKey: ", watchKey);
      }
    }
  };

  const [initial, setInitial] = useState<boolean>(true);

  useEffect(() => {
    if (initial) {
      getCurrentLocation();
      setInitial(false);
      return;
    }
    if (watchTriggerOfLocation) {
      startWatchLocation();
    } else {
      clearWatch("location");
    }
    if (watchTriggerOfDirection) {
      startWatchDirection();
    } else {
      clearWatch("direction");
    }
  }, [initial, watchTriggerOfLocation, watchTriggerOfDirection]);

  return {
    location: [currentLocation, setCurrentLocation, locationRef],
    direction: [currentDirection, setCurrentDirection, directionRef],
    watchTriggerOfLocation: [watchTriggerOfLocation, setWatchTriggerOfLocation],
    watchTriggerOfDirection: [
      watchTriggerOfDirection,
      setWatchTriggerOfDirection,
    ],
    channels,
  };
};
