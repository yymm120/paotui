import { useRef } from "react";

export const useMapRefs = () => {
  // mapRef: 地图实例
  const mapRef = useRef<any>(null);
  // markerRef: 点标记实例
  const markerRef = useRef<any>(null);
  // polygon: 多边形实例
  const polygonRef = useRef<any>(null);
  // polylineRef: 折线实例
  const polylineRef = useRef<any>(null);
  // labelRef: 文本信息实例
  const labelRef = useRef<any>(null);
  // infoWindowRef: 信息窗体实例
  const infoWindowRef = useRef<any>(null);
  return {
    mapRef,
    markerRef,
    polygonRef,
    polylineRef,
    labelRef,
    infoWindowRef,
  };
};
