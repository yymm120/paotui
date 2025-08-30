import {
  createContext,
  type Dispatch,
  type ReactNode,
  type RefObject,
  type SetStateAction,
  useRef,
  useState,
} from "react";

import {
  type LocationFeature,
  useInitialLocation,
} from "@/hooks/use-initial-location.ts";
import type { TaskLocation } from "@/types/delivery_task.ts";

export const ViewMode = {
  LIST_ONLY_VIEW: 0,
  LIST_MAP_VIEW: 1,
  MAP_ONLY_VIEW: 2,
} as const;

type Location = {
  lat: number;
  lng: number;
  // 海拔: number;
};

type ViewModeType = (typeof ViewMode)[keyof typeof ViewMode];

export type AppContextType = {
  states: AppStates;
  refs: RefObject<AppRefs>;
  features: {
    location: LocationFeature;
    map: unknown;
  };
};

export type AppRefs = {
  channels: RefObject<{ [key: string]: number }>;
  currentLocation: RefObject<Location>;
  currentDirection: RefObject<number>;
};

export type TaskAddress = {
  taskId: string;
  address_send: TaskLocation;
  address_receive: TaskLocation;
};

export type AppStates = {
  viewMode: [number, Dispatch<SetStateAction<ViewModeType>>];
  taskAddress: [
    { [key: string]: TaskAddress },
    Dispatch<SetStateAction<{ [key: string]: TaskAddress }>>,
  ];
  map: [unknown, Dispatch<SetStateAction<unknown>>];
};

export const AppContext = createContext<AppContextType>(null as any);

export const AppContextProvider = ({ children }: { children: ReactNode }) => {
  const [viewMode, setViewMode] = useState<ViewModeType>(
    ViewMode.LIST_ONLY_VIEW,
  );

  const [taskAddress, setTaskAddress] = useState<{
    [key: number]: TaskAddress;
  }>({});
  const refs = useRef<AppRefs>({} as AppRefs);
  const [map, setMap] = useState<unknown>();

  const states: AppStates = {
    viewMode: [viewMode, setViewMode],
    taskAddress: [taskAddress, setTaskAddress],
    map: [map, setMap],
  };

  const locationFeature = useInitialLocation();

  return (
    <AppContext
      value={{
        states,
        refs,
        features: {
          location: locationFeature,
          map: map,
        },
      }}
    >
      {children}
    </AppContext>
  );
};
