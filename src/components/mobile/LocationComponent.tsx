// import { useState, useEffect } from "react";
// import { Button } from "../ui/button";
// import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
// import { useTauriMobile } from "../../hooks/useTauriMobile";
// import {
//   formatCoordinates,
//   formatDistance,
// } from "../../utils/mobile";
// import { cn } from "@/lib/utils";
//
// interface LocationComponentProps {
//   className?: string;
//   showMap?: boolean;
// }
//
// export function LocationComponent({
//   className,
//   showMap = false,
// }: LocationComponentProps) {
//   const {
//     currentLocation,
//     isLocationWatching,
//     getCurrentLocationData,
//     startLocationTracking,
//     stopLocationTracking,
//     calculateDistance,
//   } = useTauriMobile();
//
//   const [nearbyDistance, setNearbyDistance] = useState<number | null>(null);
//   const [accuracy, setAccuracy] = useState<"high" | "medium" | "low">("medium");
//
//   // Update accuracy status based on location data
//   useEffect(() => {
//     if (currentLocation) {
//       if (currentLocation.accuracy <= 10) {
//         setAccuracy("high");
//       } else if (currentLocation.accuracy <= 50) {
//         setAccuracy("medium");
//       } else {
//         setAccuracy("low");
//       }
//     }
//   }, [currentLocation]);
//
//   // Calculate distance to a sample destination (for demo)
//   useEffect(() => {
//     if (currentLocation) {
//       // Sample destination coordinates (重庆解放碑)
//       const destLat = 29.5592;
//       const destLng = 106.5772;
//
//       calculateDistance(
//         currentLocation.latitude,
//         currentLocation.longitude,
//         destLat,
//         destLng,
//       ).then((distance) => {
//         setNearbyDistance(distance);
//       });
//     }
//   }, [currentLocation, calculateDistance]);
//
//   const handleRefreshLocation = async () => {
//     await getCurrentLocationData();
//   };
//
//   const handleToggleTracking = async () => {
//     if (isLocationWatching) {
//       stopLocationTracking();
//     } else {
//       await startLocationTracking();
//     }
//   };
//
//   const getAccuracyColor = (acc: typeof accuracy) => {
//     switch (acc) {
//       case "high":
//         return "text-green-600 bg-green-50 border-green-200";
//       case "medium":
//         return "text-yellow-600 bg-yellow-50 border-yellow-200";
//       case "low":
//         return "text-red-600 bg-red-50 border-red-200";
//     }
//   };
//
//   const getAccuracyText = (acc: typeof accuracy) => {
//     switch (acc) {
//       case "high":
//         return "精确定位";
//       case "medium":
//         return "一般定位";
//       case "low":
//         return "定位不准";
//     }
//   };
//
//   return (
//     <Card className={cn("w-full", className)}>
//       <CardHeader>
//         <CardTitle className="flex items-center gap-2 text-sm">
//           📍 位置信息
//           {isLocationWatching && (
//             <span className="text-xs bg-green-100 text-green-700 px-2 py-1 rounded-full">
//               实时跟踪中
//             </span>
//           )}
//         </CardTitle>
//       </CardHeader>
//       <CardContent className="space-y-4">
//         {currentLocation ? (
//           <div className="space-y-3">
//             {/* Location Coordinates */}
//             <div className="bg-gray-50 rounded-lg p-3">
//               <p className="text-xs text-gray-500 mb-1">当前坐标</p>
//               <p className="text-sm font-mono">
//                 {formatCoordinates(
//                   currentLocation.latitude,
//                   currentLocation.longitude,
//                 )}
//               </p>
//             </div>
//
//             {/* Accuracy Status */}
//             <div
//               className={cn(
//                 "rounded-lg border p-2 text-center",
//                 getAccuracyColor(accuracy),
//               )}
//             >
//               <p className="text-sm font-medium">
//                 {getAccuracyText(accuracy)} (±
//                 {Math.round(currentLocation.accuracy)}m)
//               </p>
//             </div>
//
//             {/* Distance to destination */}
//             {nearbyDistance && (
//               <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
//                 <p className="text-xs text-blue-600 mb-1">距离解放碑</p>
//                 <p className="text-sm font-medium text-blue-700">
//                   {formatDistance(nearbyDistance)}
//                 </p>
//               </div>
//             )}
//
//             {/* Last Updated */}
//             <p className="text-xs text-gray-500">
//               更新时间:{" "}
//               {new Date(currentLocation.timestamp).toLocaleTimeString("zh-CN")}
//             </p>
//           </div>
//         ) : (
//           <div className="text-center py-4">
//             <p className="text-sm text-gray-500 mb-3">📍 未获取到位置信息</p>
//             <p className="text-xs text-gray-400">点击下方按钮获取当前位置</p>
//           </div>
//         )}
//
//         {/* Control Buttons */}
//         <div className="flex gap-2">
//           <Button
//             onClick={handleRefreshLocation}
//             variant="outline"
//             size="sm"
//             className="flex-1"
//           >
//             🔄 刷新位置
//           </Button>
//           <Button
//             onClick={handleToggleTracking}
//             variant={isLocationWatching ? "destructive" : "default"}
//             size="sm"
//             className="flex-1"
//           >
//             {isLocationWatching ? "⏸️ 停止跟踪" : "▶️ 开始跟踪"}
//           </Button>
//         </div>
//
//         {/* Location Tips */}
//         <div className="text-xs text-gray-500 space-y-1">
//           <p>• 开启GPS定位以获得最佳精度</p>
//           <p>• 在室外环境下定位更准确</p>
//           <p>• 位置跟踪有助于订单派送</p>
//         </div>
//
//         {/* Map placeholder (if enabled) */}
//         {showMap && currentLocation && (
//           <div className="mt-4">
//             <div className="bg-gray-100 rounded-lg h-32 flex items-center justify-center border">
//               <div className="text-center">
//                 <p className="text-sm text-gray-500">📍 地图视图</p>
//                 <p className="text-xs text-gray-400 mt-1">
//                   集成地图组件 (开发中)
//                 </p>
//               </div>
//             </div>
//           </div>
//         )}
//       </CardContent>
//     </Card>
//   );
// }
