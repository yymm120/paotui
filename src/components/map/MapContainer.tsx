import { useRef, useState } from "react";

// import { invoke} from "@tauri-apps/api/core";

declare global {
  interface Window {
    // @ts-ignore
    // TMap: {};
  }
}

export default function MapNavigation() {
  const mapRef = useRef<HTMLDivElement>(null);
  const [routeInfo] = useState<{
    distance: string;
    duration: string;
  } | null>(null);

  // 初始化地图和导航
  // useEffect(() => {
  //   const script = document.createElement("script");
  //   script.src = `https://map.qq.com/api/gljs?v=2.exp&key=你的腾讯地图Key`;
  //   script.onload = initMap;
  //   document.head.appendChild(script);
  //
  //   function initMap() {
  //     if (!mapRef.current) return;
  //
  //     const map = new window.TMap.Map(mapRef.current, {
  //       center: new window.TMap.LatLng(39.98412, 116.307484),
  //       zoom: 13,
  //     });
  //
  //     // 路线规划服务
  //     const drivingService = new window.TMap.DrivingService({
  //       map: map,
  //       panel: mapRef.current, // 路线信息面板
  //     });
  //
  //     // 从 Rust 后端获取起点/终点（示例写死）
  //     const start = new window.TMap.LatLng(39.98412, 116.307484);
  //     const end = new window.TMap.LatLng(39.98412, 116.507484);
  //
  //     drivingService.search(start, end, (result: any) => {
  //       setRouteInfo({
  //         distance: `${result.routes[0].distance}米`,
  //         duration: `${Math.round(result.routes[0].duration / 60)}分钟`,
  //       });
  //       startVoiceNavigation(result.routes[0].steps);
  //     });
  //   }
  //
  //   return () => {
  //     document.head.removeChild(script);
  //   };
  // }, [startVoiceNavigation]);

  // 语音导航
  // function startVoiceNavigation(steps: Array<{ instruction: string }>) {
  //   steps.forEach((step, index) => {
  //     setTimeout(() => {
  //       speak(`第${index + 1}步: ${step.instruction}`);
  //     }, index * 5000); // 每5秒播报一步
  //   });
  // }

  // 语音合成
  // function speak(text: string) {
  //   if ("speechSynthesis" in window) {
  //     const utterance = new SpeechSynthesisUtterance(text);
  //     window.speechSynthesis.speak(utterance);
  //   } else {
  //     console.log("语音播报不支持");
  //   }
  // }

  return (
    <div className="flex flex-col h-screen">
      <div ref={mapRef} className="flex-1 w-full" />
      {routeInfo && (
        <div className="p-4 bg-white shadow-md">
          <p>距离: {routeInfo.distance}</p>
          <p>预计耗时: {routeInfo.duration}</p>
        </div>
      )}
    </div>
  );
}
