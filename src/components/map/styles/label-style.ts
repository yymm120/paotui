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

export type LabelStyleType = {
  //颜色属性，支持rgb()，rgba()，#RRGGBB等形式，默认为rgba(0,0,0,1)。
  color?: string;
  //描边颜色属性，支持rgb()，rgba()，#RRGGBB等形式，默认为rgba(0,0,0,0)
  strokeColor?: string;
  //文字大小属性，默认为14。
  size?: number;
  //文字偏移属性，单位为像素，以PointGeometry的位置点所对应屏幕位置为原点，x轴向右为正向左为负，y轴向下为正向上为负，默认为{x?:0, y?:0}。
  offset?: Object;
  //文字旋转属性，单位为度，以PointGeometry的位置点所对应屏幕位置为原点，逆时针为正，默认为0。
  angle?: number;
  //文字水平对齐属性，默认为center，可选值为left（文字左侧与位置锚点对齐）、right（文字右侧与位置锚点对齐）、center（文字水平中心与位置锚点对齐）。
  alignment?: string;
  //文字垂直对齐属性，默认为middle，可选值为top（文字顶部与位置点对齐）、bottom（文字底部与位置点对齐）、middle（文字垂直中心与位置点对齐）。
  verticalAlignment?: string;
  //文字背景框高度，单位为像素。
  height?: number;
  //文字背景框宽度，单位为像素。
  width?: number;
  //文字背景框内边距，单位为像素，属性支持接受1～2个值，规则符合css规范。例：“15px” 上下左右内边距为15px,//例：“15px 20px” 上下内边距为15px，左右内边距为20px,// 注意设置宽高后padding将不生效
  padding?: string;
  //文字背景框颜色属性，支持rgb()，rgba()，#RRGGBB等形式，默认为rgba(0,0,0,0)；该属性生效需要设置padding 或 width和hight。 查看示例
  backgroundColor?: string;
  //文字背景框边线宽度，单位为像素；该属性生效需要设置padding 或 width和hight。查看示例
  borderWidth?: number;
  //文字背景框圆角，单位为像素；该属性生效需要设置padding 或 width和hight。查看示例
  borderRadius?: number;
  //文字背景框边线颜色属性，支持rgb()，rgba()，#RRGGBB等形式，默认为rgba(0,0,0,0)；该属性生效需要设置padding 或 width和hight 。查看示例
  borderColor?: string;
  //文本自动换行，支持设置软换行和硬换行，软换行支持配置最大换行宽度，硬换行换行符为’\n’。 LabelWrapOptions如果为空对象则以文本中换行符进行换行，如果为null或undefined则不换行
  wrapOptions?: LabelWrapOptions;
};

export const label_styles: { [key: string]: LabelStyleType } = {
  multiLabelStyle1: {
    color: "#3777FF",
    size: 20,
  },
  multiLabelStyle2: {
    color: "#D54440",
    size: 20,
  },
};
