// import { useState } from "react";
// import { Button } from "../ui/button";
// import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
// import { useTauriMobile } from "../../hooks/useTauriMobile";
// import { cn } from "@/lib/utils";
//
// interface CameraComponentProps {
//   orderId: string;
//   onPhotoTaken?: (photoPath: string) => void;
//   className?: string;
// }

// export function CameraComponent({
//   orderId,
//   onPhotoTaken,
//   className,
// }: CameraComponentProps) {
//   const [isLoading, setIsLoading] = useState(false);
//   const [photoPath, setPhotoPath] = useState<string | null>(null);
//   const { takeDeliveryPhoto, sendMobileNotification } = useTauriMobile();
//
//   const handleTakePhoto = async () => {
//     setIsLoading(true);
//     try {
//       const savedPath = await takeDeliveryPhoto(orderId);
//       if (savedPath) {
//         setPhotoPath(savedPath);
//         onPhotoTaken?.(savedPath);
//         await sendMobileNotification(
//           "送达照片已保存",
//           "订单送达照片已成功保存到本地",
//         );
//       }
//     } catch (error) {
//       console.error("Failed to take photo:", error);
//       await sendMobileNotification("拍照失败", "无法保存送达照片，请重试");
//     } finally {
//       setIsLoading(false);
//     }
//   };
//
//   return (
//     <Card className={cn("w-full", className)}>
//       <CardHeader>
//         <CardTitle className="flex items-center gap-2 text-sm">
//           📷 送达拍照
//         </CardTitle>
//       </CardHeader>
//       <CardContent className="space-y-4">
//         {photoPath ? (
//           <div className="space-y-3">
//             <div className="rounded-lg bg-green-50 border border-green-200 p-3">
//               <p className="text-sm text-green-700">✅ 送达照片已保存</p>
//               <p className="text-xs text-green-600 mt-1">
//                 文件路径: {photoPath}
//               </p>
//             </div>
//             <Button
//               onClick={handleTakePhoto}
//               variant="outline"
//               size="sm"
//               disabled={isLoading}
//               className="w-full"
//             >
//               {isLoading ? "处理中..." : "重新拍照"}
//             </Button>
//           </div>
//         ) : (
//           <div className="space-y-3">
//             <p className="text-sm text-gray-600">
//               为订单 {orderId.slice(-8)} 拍摄送达照片
//             </p>
//             <Button
//               onClick={handleTakePhoto}
//               disabled={isLoading}
//               className="w-full bg-blue-600 hover:bg-blue-700"
//             >
//               {isLoading ? "拍照中..." : "📷 拍摄送达照片"}
//             </Button>
//           </div>
//         )}
//
//         <div className="text-xs text-gray-500 space-y-1">
//           <p>• 请确保照片清晰显示送达地点</p>
//           <p>• 照片将自动保存到本地存储</p>
//           <p>• 可作为送达凭证使用</p>
//         </div>
//       </CardContent>
//     </Card>
//   );
// }
