type PolygonStyleType = {
  // 面填充色，支持rgb()，rgba()，#RRGGBB等形式，默认为rgba(55,119,255,0.16)。
  color?: string;
  // 是否显示边线，默认为true。
  showBorder?: boolean;
  // 边线颜色，支持rgb()，rgba()，#RRGGBB等形式，默认为#3777FF，showBorder为true时有效。
  borderColor?: string;
  // 边线宽度，正整数，单位为像素，指的是地图pitch为0时的屏幕像素大小，如果pitch不为0，实际绘制出来的线宽度与屏幕像素会存在一定误差，默认为2，showBorder为true时有效。
  borderWidth?: number;
  // 边线虚线虚线展示方式，[0, 0]为实线，[10, 10]表示十个像素的实线和十个像素的空白（如此反复）组成的虚线，默认为[0, 0];这里的像素指的是地图pitch为0时的屏幕像素大小，如果pitch不为0，实际绘制出来的线宽度与屏幕像素会存在一定误差。
  borderDashArray?: number[];
};

export const polygon_styles: { [key: string]: PolygonStyleType } = {
  polygon1: {
    color: "rgba(255, 73, 0, 0.2)",
    borderColor: "#FF4900",
    showBorder: true,
  },
  polygon2: {
    color: "rgba(255, 206, 0, 0.2)",
    borderColor: "#FC0",
    showBorder: true,
  },
};
