export type ArrowOptions = {
  // 箭头图标宽度，单位为px，默认为6，最大支持255
  width?: number;
  //	箭头图标高度（沿线方向长度），单位为px，默认为4，最大支持255
  height?: number;
  //	箭头图标之间的孔隙长度，单位为px，默认为50，最大支持255
  space?: number;
  //	箭头动态移动的速度，默认为0，静止不动，单位为(像素/秒)
  animSpeed?: number;
};

export type PolylineStyleType = {
  // 线填充色，支持rgb()，rgba()，#RRGGBB等形式，默认为#3777FF。
  color?: string;
  //	折线宽度，正整数，单位为像素，指的是地图pitch为0时的屏幕像素大小，如果pitch不为0，实际绘制出来的线宽度与屏幕像素会存在一定误差，默认为3。
  width?: number;
  // 边线宽度，非负整数，默认为0，单位为像素，指的是地图pitch为0时的屏幕像素大小，如果pitch不为0，实际绘制出来的线宽度与屏幕像素会存在一定误差，默认为1。
  borderWidth?: number;
  // 边线颜色，支持rgb()，rgba()，#RRGGBB等形式，borderWidth不为0时有效，默认为#3777FF。
  borderColor?: string;
  // 擦除线填充色，支持rgb()，rgba()，#RRGGBB等形式，默认为#3777FF。
  eraseColor?: string;
  // 线端头方式，可选为butt，round，square，默认为butt。
  lineCap?: string;
  // 虚线展示方式，[0, 0]为实线，[10, 10]表示十个像素的实线和十个像素的空白（如此反复）组成的虚线，默认为[0, 0];这里的像素指的是地图pitch为0时的屏幕像素大小，如果pitch不为0，实际绘制出来的线宽度与屏幕像素会存在一定误差。
  dashArray?: number[];
  // 是否沿线路方向显示箭头，默认为false，建议线宽度大于6时使用。
  showArrow?: boolean;
  // 箭头显示配置，仅在showArrow为true时有效。
  arrowOptions?: ArrowOptions;
  // 是否对折线启用泛光效果，需在MapRenderOptions中开启泛光后才可使用。查看示例
  enableBloom?: boolean;
};

export const polyline_styles: { [key: string]: PolylineStyleType } = {
  polyline: {
    color: "#2C68FF",
    width: 5,
    borderWidth: 0,
  },
};
