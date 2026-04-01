# pos_trans

## 函数签名

```rust
pub fn pos_trans(src: &RgbImage, tx: i32, ty: i32) -> RgbImage
```

## 功能描述

对图像进行平移变换（Translation），将图像在水平和垂直方向上移动指定的像素距离。超出边界的部分填充黑色。

## 参数说明

- `src: &RgbImage` - 输入的 RGB 图像引用
- `tx: i32` - X 方向（水平）平移量
  - 正值：向右平移
  - 负值：向左平移
- `ty: i32` - Y 方向（垂直）平移量
  - 正值：向下平移
  - 负值：向上平移

## 返回值

- `RgbImage` - 平移后的图像，尺寸与原图相同

## 数学原理

### 1. 平移变换矩阵

二维平移的齐次坐标变换：

```
[x']   [1  0  tx] [x]
[y'] = [0  1  ty] [y]
[1 ]   [0  0  1 ] [1]
```

简化为：
```
x' = x + tx
y' = y + ty
```

### 2. 逆向映射

在图像处理中通常使用逆向映射（后向映射）：
- 对于目标图像的每个像素 `(x', y')`
- 计算其在源图像中的位置 `(x, y)`
- `x = x' - tx`
- `y = y' - ty`

### 3. 边界处理

如果映射后的坐标超出源图像边界：
- `x < 0` 或 `x >= width` 或 `y < 0` 或 `y >= height`
- 则该像素设为黑色 `[0, 0, 0]`

## 实现逻辑

### 第 6-9 行：初始化目标图像（黑色背景）
```rust
let mut dst = RgbImage::new(src.width(), src.height());
for pixel in dst.pixels_mut() {
    *pixel = image::Rgb([0, 0, 0]);
}
```
**原理**：
- 创建与源图像相同尺寸的空白图像
- 将所有像素初始化为黑色 `[0, 0, 0]`
- 超出边界的区域将保持黑色

---

### 第 10-22 行：遍历源图像并平移
```rust
for y in 0..src.height() {
    for x in 0..src.width() {
        let target_x = x as i32 + tx;
        let target_y = y as i32 + ty;
        if target_x >= 0
            && target_x < src.width() as i32
            && target_y >= 0
            && target_y < src.height() as i32
        {
            dst.put_pixel(target_x as u32, target_y as u32, *src.get_pixel(x, y));
        }
    }
}
```
**原理**：

**第 10-11 行**：遍历源图像的所有像素

**第 12-13 行**：计算目标坐标
```rust
let target_x = x as i32 + tx;
let target_y = y as i32 + ty;
```
- 使用有符号整数（`i32`）以支持负坐标
- 向前映射：源坐标 + 平移量 = 目标坐标

**示例**：
- 源像素 `(100, 50)`，平移 `tx=30, ty=-20`
- 目标坐标：`(130, 30)`

**第 14-18 行**：边界检查
```rust
if target_x >= 0
    && target_x < src.width() as i32
    && target_y >= 0
    && target_y < src.height() as i32
```
- 检查目标坐标是否在图像边界内
- 四个条件：
  - `target_x >= 0`: 不超出左边界
  - `target_x < width`: 不超出右边界
  - `target_y >= 0`: 不超出上边界
  - `target_y < height`: 不超出下边界

**第 19 行**：复制像素
```rust
dst.put_pixel(target_x as u32, target_y as u32, *src.get_pixel(x, y));
```
- 从源图像读取像素值
- 写入目标图像的新坐标
- `*` 解引用 `Rgb` 类型

**遗漏处理**：
- 如果目标坐标超出边界，则不写入（保持黑色）
- 这会在图像边缘产生黑色区域

---

### 第 23 行：返回结果
```rust
dst
```

## 平移效果示例

### 示例 1：向右下平移
```rust
let translated = pos_trans(&img, 50, 30);
// 图像向右移动 50 像素，向下移动 30 像素
// 左侧和上侧出现黑色边缘
```

```
原图：         平移后：
┌─────┐       ┌─────┐
│IMAGE│  →    │█████│
│     │       │█IMA█│
└─────┘       └─────┘
```
（█ 表示黑色区域）

### 示例 2：向左上平移
```rust
let translated = pos_trans(&img, -50, -30);
// 图像向左移动 50 像素，向上移动 30 像素
// 右侧和下侧出现黑色边缘
```

```
原图：         平移后：
┌─────┐       ┌─────┐
│IMAGE│  →    │GE███│
│     │       │█████│
└─────┘       └─────┘
```

## 应用场景

### 1. 图像配准
```rust
// 将多张图像对齐到相同位置
let aligned1 = pos_trans(&img1, offset_x1, offset_y1);
let aligned2 = pos_trans(&img2, offset_x2, offset_y2);
```

### 2. 数据增强
```rust
// 机器学习训练时的数据增强
let augmented1 = pos_trans(&img, 10, 0);
let augmented2 = pos_trans(&img, -10, 5);
```

### 3. 图像拼接预处理
```rust
// 移动图像到拼接画布的指定位置
let positioned = pos_trans(&img, canvas_x, canvas_y);
```

### 4. 手抖校正
```rust
// 根据运动估计结果校正相机抖动
let stabilized = pos_trans(&frame, -motion_x, -motion_y);
```

## 与其他几何变换的对比

| 变换类型 | 公式 | 参数数量 | 保持性质 |
|---------|------|----------|----------|
| 平移 | `(x', y') = (x+tx, y+ty)` | 2 | 形状、大小、方向 |
| 旋转 | `(x', y') = R(θ)·(x,y)` | 1 | 形状、大小 |
| 缩放 | `(x', y') = (sx·x, sy·y)` | 2 | 形状、方向 |
| 剪切 | `(x', y') = (x+k·y, y)` | 1-2 | 面积 |
| 仿射 | `(x', y') = A·(x,y) + b` | 6 | 平行线 |

## 性能优化建议

当前实现使用前向映射，可能导致"空洞"（holes）问题。改进方法：

### 逆向映射实现
```rust
// 对于目标图像的每个像素
for y' in 0..dst.height() {
    for x' in 0..dst.width() {
        let src_x = x' as i32 - tx;
        let src_y = y' as i32 - ty;
        if src_x >= 0 && src_x < width && src_y >= 0 && src_y < height {
            dst.put_pixel(x', y', *src.get_pixel(src_x as u32, src_y as u32));
        }
    }
}
```

**优点**：
- 保证目标图像每个像素都被赋值
- 避免空洞问题

## 使用示例

```rust
let img = image::open("photo.jpg")?.to_rgb8();

// 向右平移 100 像素
let right = pos_trans(&img, 100, 0);
right.save("shifted_right.jpg")?;

// 向左下平移
let left_down = pos_trans(&img, -50, 80);
left_down.save("shifted_left_down.jpg")?;

// 组合变换：先平移再旋转
let translated = pos_trans(&img, 50, 50);
let rotated = img_rotate(&translated, 45.0);
```

## 性能说明

- 时间复杂度：O(W×H)
  - 遍历所有像素一次
- 空间复杂度：O(W×H)
  - 输出图像
- 非常高效，无浮点运算
- 边界检查是主要开销
