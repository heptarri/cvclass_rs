# draw_histogram

## 函数签名

```rust
pub fn draw_histogram(hist: &[u32; 256]) -> RgbImage
```

## 功能描述

将直方图数据可视化为图像，生成一个带有白色柱状图的黑色背景图像。适用于直观观察图像的灰度分布情况。

## 参数说明

- `hist: &[u32; 256]` - 输入的直方图数组引用，通常由 `calculate_histogram()` 生成

## 返回值

- `RgbImage` - 512×400 像素的 RGB 图像，其中：
  - 背景为黑色 `[0, 0, 0]`
  - 柱状图为白色 `[255, 255, 255]`

## 数学原理

柱状图绘制涉及归一化映射：

```
y_normalized = (count / max_count) × (height - 1)
```

其中：
- `count` 是某个灰度级的频次
- `max_count` 是所有灰度级中的最大频次
- `height` 是图像高度（400 像素）
- 归一化确保最高的柱子正好触及顶部

## 实现逻辑

### 第 16-18 行：初始化画布参数
```rust
let hist_h = 400;
let hist_w = 512;
let mut img = RgbImage::new(hist_w, hist_h);
```
**原理**：
- 创建 512×400 像素的空白 RGB 图像
- 宽度 512 像素可以容纳 256 个灰度级，每个占 2 像素宽度

---

### 第 19-21 行：填充黑色背景
```rust
for pixel in img.pixels_mut() {
    *pixel = Rgb([0, 0, 0]);
}
```
**原理**：
- 遍历所有像素，设置为黑色 `[R=0, G=0, B=0]`
- 为后续绘制白色柱状图提供对比背景

---

### 第 23-25 行：计算归一化参数
```rust
let max_count = hist.iter().copied().max().unwrap_or(1) as f32;
let bin_w = hist_w / 256;
let bin_w = bin_w.max(1);
```
**原理**：
- **第 23 行**：找到直方图中的最大频次
  - `.unwrap_or(1)` 防止空直方图导致的除零错误
  - 转换为 `f32` 用于浮点运算
- **第 24 行**：计算每个灰度级的显示宽度
  - `512 / 256 = 2` 像素每柱
- **第 25 行**：确保至少为 1 像素宽（避免不可见）

---

### 第 27-38 行：绘制柱状图
```rust
for (i, &count) in hist.iter().enumerate() {
    let normalized = ((count as f32 / max_count) * (hist_h as f32 - 1.0)).round() as u32;
    let x0 = i as u32 * bin_w;
    let x1 = (i as u32 + 1) * bin_w;
    let x1 = x1.min(hist_w);
    for x in x0..x1 {
        for y in (hist_h - normalized)..hist_h {
            img.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }
}
```
**原理**：

- **第 27 行**：遍历所有 256 个灰度级及其频次
  - `enumerate()` 提供索引 `i` 和值 `count`

- **第 28 行**：归一化高度
  - 公式：`normalized = (count / max_count) × (height - 1)`
  - 示例：如果 `count = max_count`，则 `normalized = 399`（满高度）
  - `.round()` 四舍五入到最近的整数像素

- **第 29-31 行**：计算柱子的水平范围
  - `x0` 是柱子的左边界：`i × 2`
  - `x1` 是柱子的右边界：`(i + 1) × 2`
  - `.min(hist_w)` 防止最后一个柱子超出图像边界

- **第 32-34 行**：填充柱子区域
  - 外层循环：遍历柱子的宽度范围 `[x0, x1)`
  - 内层循环：从底部向上填充到归一化高度
    - 起始 y 坐标：`hist_h - normalized`（顶部）
    - 结束 y 坐标：`hist_h`（底部）
  - 将像素设置为白色 `[255, 255, 255]`

**坐标系说明**：
- 图像坐标系原点在左上角
- Y 轴向下递增
- 因此柱状图从底部（y=400）向上（y减小）绘制

---

### 第 39 行：返回图像
```rust
img
```
返回绘制完成的直方图图像。

## 视觉效果

生成的图像特征：
- **宽度**：512 像素（256 个柱子，每个 2 像素宽）
- **高度**：400 像素
- **颜色**：白色柱状图 + 黑色背景
- **柱子高度**：与频次成正比，最高柱触及顶部

## 使用示例

```rust
let gray_img = to_grayscale(&rgb_img);
let hist = calculate_histogram(&gray_img);
let hist_img = draw_histogram(&hist);
hist_img.save("histogram.png")?;
```

## 扩展建议

可以增强的功能：
1. 添加坐标轴和刻度
2. 用不同颜色区分亮部/暗部/中间调
3. 支持对数尺度显示（便于观察小频次）
4. 叠加累积分布函数（CDF）曲线
