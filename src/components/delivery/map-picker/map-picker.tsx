import React, { useState, useEffect } from "react";

/*
*
* 已选择的位置
* 名称: 卓然·假日之城10栋

* 地址: 重庆市合川区假日大道卓然·假日之城

* 经纬度: 30.000234, 106.242553

* 城市: 重庆市
*
* */
interface LocationData {
  module: "locationPicker";
  latlng: {
    lat: number;
    lng: number;
  };
  poiaddress: string;
  poiname: string;
  cityname: string;
}

interface MapPickerProps {
  visible: boolean;
  onLocationSelect: (location: LocationData) => void;
  onClose: () => void;
  apiKey: string;
  referer: string;
}

const MapPicker: React.FC<MapPickerProps> = ({
  visible,
  onLocationSelect,
  onClose,
  apiKey,
  referer,
}) => {
  const [iframeLoaded, setIframeLoaded] = useState(false);

  useEffect(() => {
    const handleMessage = (event: MessageEvent) => {
      const loc = event.data;
      if (loc?.module === "locationPicker") {
        onLocationSelect(loc);
        onClose();
      }
    };

    window.addEventListener("message", handleMessage);
    return () => window.removeEventListener("message", handleMessage);
  }, [onLocationSelect, onClose]);

  if (!visible) return null;

  const iframeUrl = `https://apis.map.qq.com/tools/locpicker?search=1&type=1&key=${apiKey}&referer=${referer}`;

  return (
    <div className="fixed inset-0 z-50 bg-black bg-opacity-50 flex flex-col">
      <div className="bg-white p-4 flex justify-between items-center">
        <h2 className="text-lg font-semibold">选择位置</h2>
        <button onClick={onClose} className="text-gray-500 hover:text-gray-700">
          ×
        </button>
      </div>
      <div className="flex-1 relative">
        <iframe
          id="mapPickerIframe"
          src={iframeUrl}
          className="w-full h-full"
          allow="geolocation"
          onLoad={() => setIframeLoaded(true)}
        />
        {!iframeLoaded && (
          <div className="absolute inset-0 flex items-center justify-center bg-gray-100">
            <p>地图加载中...</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default MapPicker;
