import {
  BaseMap,
  MultiLabel,
  MultiPolygon,
  MultiPolyline,
} from "tlbs-map-react";
import {
  type ReactElement,
  type MouseEvent,
  useCallback,
  useState,
  createContext,
  type Dispatch,
  type SetStateAction,
  useContext,
  type RefObject,
  type ReactNode,
} from "react";
import { marker_styles } from "@/components/map/styles/marker-style.ts";
import { polygon_styles } from "@/components/map/styles/polygon-style.ts";
import { polyline_styles } from "@/components/map/styles/polyline-style.ts";

export const TMapApi = (window as any).TMap;

import { MultiMarker } from "tlbs-map-react";
import { Button } from "@/components/ui/button.tsx";
import { cn } from "@/lib/utils.ts";
import { useMapRefs } from "@/hooks/use-map-refs.ts";
import { label_styles } from "@/components/map/styles/label-style.ts";
import type { CustomMapOptions } from "tlbs-map-react/dist/interfaces";
import { AppContext } from "@/components/app-context.tsx";

export type TencentMapContainerProps = {
  children?: ReactElement[];
  display?: boolean;
  className?: string;
};

export type TencentMapTriggerProps = {
  children?: ReactNode;
  className?: string;
  onClick?: (event?: MouseEvent<HTMLDivElement, MouseEvent>) => void;
  hidden?: boolean;
};

export type TencentMapContextType = {
  display: [boolean, Dispatch<SetStateAction<boolean>>];
  refs: {
    mapRef: RefObject<any>;
    markerRef: RefObject<any>;
    polygonRef: RefObject<any>;
    polylineRef: RefObject<any>;
    labelRef: RefObject<any>;
    infoWindowRef: RefObject<any>;
  };
  geometries: {
    marker: [any[], Dispatch<SetStateAction<any[]>>];
    polygon: [any[], Dispatch<SetStateAction<any[]>>];
    polyline: [any[], Dispatch<SetStateAction<any[]>>];
    label: [any[], Dispatch<SetStateAction<any[]>>];
  };
};

export type TencentMapProps = {
  className?: string;
  children?: ReactElement;
  width?: number;
  height?: number;
  mapOptions?: CustomMapOptions;
  onMapInitialed: (feature: unknown) => void;
};

const TencentMapContext = createContext<TencentMapContextType>({} as any);

export const TencentMapContainer = ({
  children,
  display = undefined,
  className,
}: TencentMapContainerProps) => {
  const [displayMap, setDisplayMap] = useState(false);

  const {
    mapRef,
    markerRef,
    polygonRef,
    polylineRef,
    labelRef,
    infoWindowRef,
  } = useMapRefs();

  const [markerGeometriesData, setMarkerGeometriesData] = useState<any[]>([]);
  const [polygonGeometriesData, setPolygonGeometriesData] = useState<any[]>([]);
  const [polylineGeometriesData, setPolylineGeometriesData] = useState<any[]>(
    [],
  );
  const [labelGeometriesData, setLabelGeometriesData] = useState<any[]>([]);

  return (
    <TencentMapContext
      value={{
        display: [display !== undefined ? display : displayMap, setDisplayMap],
        refs: {
          mapRef,
          markerRef,
          polygonRef,
          polylineRef,
          labelRef,
          infoWindowRef,
        },
        geometries: {
          marker: [markerGeometriesData, setMarkerGeometriesData],
          polygon: [polygonGeometriesData, setPolygonGeometriesData],
          polyline: [polylineGeometriesData, setPolylineGeometriesData],
          label: [labelGeometriesData, setLabelGeometriesData],
        },
      }}
    >
      <div className={cn(className)}>{children}</div>
    </TencentMapContext>
  );
};

export const TencentMapTrigger = ({
  children,
  className,
  onClick,
  hidden,
}: TencentMapTriggerProps) => {
  const {
    display: [displayMap, setDisplayMap],
  } = useContext(TencentMapContext);

  return (
    <>
      <Button
        hidden={hidden}
        className={cn(className)}
        onClick={() => (onClick ? onClick() : setDisplayMap(!displayMap))}
      >
        {children}
      </Button>
    </>
  );
};

export const TencentMap = ({
  children,
  className,
  mapOptions,
  onMapInitialed,
}: TencentMapProps) => {
  const mapContext = useContext(TencentMapContext);
  const {
    display: [displayMap],
    refs: {
      mapRef,
      markerRef,
      polygonRef,
      polylineRef,
      labelRef,
      // infoWindowRef,
    },
    geometries: {
      marker: [markerGeometriesData, setMarkerGeometriesData],
      polygon: [polygonGeometriesData, setPolygonGeometriesData],
      polyline: [polylineGeometriesData, setPolylineGeometriesData],
      label: [labelGeometriesData, setLabelGeometriesData],
    },
  } = mapContext;

  const onMapInitialedExtend = useCallback(() => {
    onMapInitialed(mapContext);
  }, [onMapInitialed, mapContext]);

  const [center, setCenter] = useState({ lat: 40.0404, lng: 116.2735 });
  const [showControl, setShowControl] = useState(true);
  // const [geometries, setGeometries] = useState(geometriesData);

  const markerClickHandler = useCallback(
    (e: any) => {
      console.log("🚀🚀🚀 触发点击事件", e, markerGeometriesData);
    },
    [markerGeometriesData],
  );

  const polygonClickHandler = useCallback(
    (e: any) => {
      console.log("🚀🚀🚀 触发点击事件", e, polygonGeometriesData);
    },
    [polygonGeometriesData],
  );

  const polylineClickHandler = useCallback(
    (e: any) => {
      console.log("🚀🚀🚀 触发点击事件", e, polylineGeometriesData);
    },
    [polylineGeometriesData],
  );

  const labelClickHandler = useCallback(
    (e: any) => {
      console.log("🚀🚀🚀 触发点击事件", e, labelGeometriesData);
    },
    [labelGeometriesData],
  );

  return (
    <>
      {displayMap && (
        <BaseMap
          className={cn("relative", className)}
          ref={mapRef}
          style={{ width: "100%", height: "100%" }}
          apiKey="44OBZ-NDTOT-LRJXQ-VQ7TW-2YTZT-K4FR5"
          control={{
            zoom: {
              position: "topRight",
              className: "tmap-zoom-control-box",
              numVisible: true,
            },
          }}
          options={mapOptions}
          onMapInited={onMapInitialedExtend}
        >
          <MultiMarker
            ref={markerRef}
            styles={marker_styles}
            geometries={markerGeometriesData}
            onClick={markerClickHandler}
          />
          <MultiPolygon
            ref={polygonRef}
            styles={polygon_styles}
            geometries={polygonGeometriesData}
            onClick={polygonClickHandler}
          />
          <MultiPolyline
            ref={polylineRef}
            styles={polyline_styles}
            geometries={polylineGeometriesData}
            onClick={polylineClickHandler}
          />
          <MultiLabel
            ref={labelRef}
            styles={label_styles}
            geometries={labelGeometriesData}
            onClick={labelClickHandler}
          />
          {/*<InfoWindow ref={infoWindowRef} visible={visible} position={position} content='Hello, world !' onCloseclick={() => {*/}
          {/*    setVisible(false);*/}
          {/*    console.log('🚀🚀🚀 关闭信息窗体');*/}
          {/*  }}*/}
          {/*/>*/}

          <div className={"w-full h-full left-0 top-0 absolute"}>
            {children}
          </div>
        </BaseMap>
      )}
    </>
  );
};
