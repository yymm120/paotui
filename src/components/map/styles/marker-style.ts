/**
 * Marker的图像、文本随地图放大缩小的控制参数。
 */
type RelativeScaleOptions = {
  // 设置marker图片宽高像素单位与zoom级别的瓦片像素相同，如当设置为18时，zoom小于18marker会被缩小直至达到minScale设置的最小缩放倍数，大于时marker直至达到maxScale设置的最大缩放倍数；enableRelativeScale为true时生效，默认18
  scaleZoom?: number;
  // 设置最小的缩放比例，范围 [0, 1)，默认为0.5
  minScale?: number;
  // 设置最大的缩放比例，范围 [1, 10)， 默认为1
  maxScale?: number;
};

/**
 * 文本换行配置。当同时指定了最大换行宽度、最大行数和换行符时，换行优先级为?: 最大行数>换行符>最大换行宽度。
 */
type LabelWrapOptions = {
  // 最大换行宽度，单位是像素，默认不限制
  maxWidth?: number;
  // 最大行数，默认不限制
  maxLineCount?: number;
  // 行间距，单位是像素，默认为0
  rowSpacing?: number;
};

export type MarkerStyleType = {
  // 必需，标注点图片的宽度，默认为34
  width?: number;
  // 必需，标注点图片的高度，默认为50
  height?: number;
  // 标注点图片的锚点位置，对象字面量{x:number, y:number}形式，在地图各种操作中，锚点的位置与标注位置点是永远对应的；若没有锚点默认为{ x: width/2, y: height }；锚点以图片左上角点为原点
  anchor?: { x: number; y: number };
  // 标注点图片url或base64地址，若为url地址图片一定要在服务器端配置允许跨域
  src?: string;
  // 标注点图片的朝向，可取’map’（贴地）或’screen’（直立），默认为’screen’。
  faceTo?: string;
  // 是否启动随地图放大缩小，默认为false（只针对非碰撞Marker生效）
  enableRelativeScale?: boolean;
  // Marker的图像、文本随地图放大缩小的控制参数（只针对非碰撞Marker生效）
  relativeScaleOptions?: RelativeScaleOptions;
  // 标注点图片的旋转角度，单位为度，非负数；以锚点为旋转原点，逆时针为正。
  rotate?: number;
  // 标注点图片的透明度，取值0-1。
  opacity?: number;
  // 标注点文本颜色属性，支持rgb()，rgba()，#RRGGBB等形式，默认为rgba(0,0,0,1) 。
  color?: string;
  // 标注点文本描边颜色属性，支持rgb()，rgba()，#RRGGBB等形式，默认为rgba(0,0,0,0)。
  strokeColor?: string;
  // 标注点文本描边宽度，默认为1。
  strokeWidth?: number;
  // 标注点文本文字大小属性，默认为14。
  size?: number;
  // 标注点文本文字相对于标注点图片的方位，可选位于标注点图片的center，top，bottom，left，right方位，默认位于图片的中心center 。
  direction?: string;
  // 标注点文本文字基于direction方位的偏移量，单位为像素，以文本文字中心为原点，x轴向右为正向左为负，y轴向下为正向上为负，默认为{x?:0, y?:0} 。
  offset?: { x: number; y: number };
  // 标注点文本自动换行，支持设置软换行和硬换行，软换行支持配置最大换行宽度，硬换行换行符为’\n’。 LabelWrapOptions如果为空对象则以文本中换行符进行换行，如果为null或undefined则不换行
  wrapOptions?: LabelWrapOptions;
  // 标注点文字背景框内边距，单位为像素，属性支持接受1～2个值，规则符合css规范。例：“15px” 上下左右内边距为15px例：“15px 20px” 上下内边距为15px，左右内边距为20px, 注意设置背景宽高后padding将不生效
  padding?: string;
  // 标注点文本背景的宽度，单位为像素，默认为0
  backgroundWidth?: number;
  // 标注点文本背景的高度，单位为像素，默认为0
  backgroundHeight?: number;
  //	标注点文本背景颜色属性，支持rgb()，rgba()，#RRGGBB等形式，该属性生效需要设置padding 或 backgroundWidth和backgroundHeight，默认为rgba(0,0,0,1)
  backgroundColor?: string;
  // 标注点文字背景框边线宽度，单位为像素；该属性生效需要设置padding 或 backgroundWidth和backgroundHeight，默认为0
  backgroundBorderWidth?: number;
  //	标注点文本背景框圆角，单位为像素；该属性生效需要设置padding 或 backgroundWidth和backgroundHeight，默认为0
  backgroundBorderRadius?: number;
  // 标注点文本背景描边颜色属性，支持rgb()，rgba()，#RRGGBB等形式，该属性生效需要设置padding 或 backgroundWidth和backgroundHeight，默认为rgba(0,0,0,0)
  backgroundBorderColor?: string;
};

export const marker_styles: { [key: string]: MarkerStyleType } = {
  multiMarkerStyle1: {
    width: 20,
    height: 30,
    anchor: { x: 10, y: 30 },
  },
  multiMarkerStyle2: {
    width: 20,
    height: 30,
    anchor: { x: 10, y: 30 },
    src: "https://mapapi.qq.com/web/lbs/javascriptGL/demo/img/markerDefault.png",
  },
};
