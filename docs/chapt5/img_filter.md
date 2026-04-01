# img_filter

## 函数签名

```rust
pub fn img_filter<const H: usize, const W: usize>(
    src: &GrayImage,
    mask: &[[f32; W]; H],
) -> GrayImage
```

## 功能描述

通用的空间域滤波函数，使用指定的卷积核对灰度图像进行滤波处理。这是实现各种图像平滑、锐化和边缘检测算法的基础函数。

## 参数说明

- `src: &GrayImage` - 输入的灰度图像引用
- `mask: &[[f32; W]; H]` - 卷积核（滤波器）
  - `H`：核的高度（编译期常量）
  - `W`：核的宽度（编译期常量）
  - 元素为浮点数，支持小数和负数权重

## 返回值

- `GrayImage` - 滤波后的灰度图像

## 数学原理

### 1. 二维卷积

空间域滤波本质上是二维离散卷积：

```
g(x, y) = Σ Σ f(x+i, y+j) · h(i, j)
          i j
```

其中：
- `f(x, y)` 是输入图像
- `g(x, y)` 是输出图像  
- `h(i, j)` 是卷积核（mask）
- 求和范围覆盖整个卷积核

### 2. 归一化

为了保持图像亮度，通常需要归一化：

```
g(x, y) = [Σ Σ f(x+i, y+j) · h(i, j)] / Σ Σ h(i, j)
           i j                          i j
```

**特殊情况**：
- 如果 `Σh(i,j) = 0`（如边缘检测核），则不归一化
- 如果 `Σh(i,j) = 1`（已归一化的核），直接使用

### 3. 边界处理

当卷积核覆盖图像边界外时，使用 **边缘复制**（Edge Clamp）策略：
- 超出边界的坐标被限制到最近的有效坐标
- `x < 0` → `x = 0`
- `x >= width` → `x = width - 1`

## 实现逻辑

### 第 9-14 行：初始化参数
```rust
let mut dst = GrayImage::new(src.width(), src.height());
let width = src.width() as i32;
let height = src.height() as i32;
let half_w = (W / 2) as i32;
let half_h = (H / 2) as i32;
let weight_sum: f32 = mask.iter().flat_map(|row| row.iter()).copied().sum();
```
**原理**：

- **第 9 行**：创建输出图像

- **第 10-11 行**：图像尺寸转换为有符号整数
  - 用于处理负坐标（边界外）

- **第 12-13 行**：计算卷积核的半径
  - `half_w = W / 2`（整数除法）
  - 示例：3×3 核 → `half_w = 1, half_h = 1`
  - 5×5 核 → `half_w = 2, half_h = 2`
  - 用于将核中心对齐到当前像素

- **第 14 行**：计算卷积核权重总和
  - `.iter()` 遍历行
  - `.flat_map(|row| row.iter())` 展平为单层迭代器
  - `.copied().sum()` 求和
  - 用于归一化，避免改变图像亮度

---

### 第 16-39 行：主循环 - 卷积运算
```rust
for y in 0..height {
    for x in 0..width {
        let mut accum = 0.0f32;
        for my in 0..H as i32 {
            for mx in 0..W as i32 {
                let px = x + mx - half_w;
                let py = y + my - half_h;
                let sx = px.clamp(0, width - 1) as u32;
                let sy = py.clamp(0, height - 1) as u32;
                let weight = mask[my as usize][mx as usize];
                let pixel = src.get_pixel(sx, sy).0[0] as f32;
                accum += pixel * weight;
            }
        }

        let value = if weight_sum == 0.0 {
            accum
        } else {
            accum / weight_sum
        };
        let gray = value.round().clamp(0.0, 255.0) as u8;
        dst.put_pixel(x as u32, y as u32, image::Luma([gray]));
    }
}
```
**原理**：

**第 16-17 行**：遍历输出图像的每个像素坐标

**第 18 行**：初始化累加器
```rust
let mut accum = 0.0f32;
```
- 存储卷积结果（加权和）

**第 19-29 行**：遍历卷积核并累加
```rust
for my in 0..H as i32 {  // 核的行
    for mx in 0..W as i32 {  // 核的列
```

**第 21-22 行**：计算源图像采样坐标
```rust
let px = x + mx - half_w;
let py = y + my - half_h;
```
- 将核中心对齐到当前像素 `(x, y)`
- 示例（3×3 核，half_w=1, half_h=1）：
  - `mx=0, my=0`: `px = x-1, py = y-1` （左上）
  - `mx=1, my=1`: `px = x, py = y` （中心）
  - `mx=2, my=2`: `px = x+1, py = y+1` （右下）

**第 23-24 行**：边界处理（边缘复制）
```rust
let sx = px.clamp(0, width - 1) as u32;
let sy = py.clamp(0, height - 1) as u32;
```
- `.clamp(0, width-1)` 将坐标限制在有效范围内
- 示例：如果 `px = -1`，则 `sx = 0`
- 示例：如果 `px = 640`（图像宽度 640），则 `sx = 639`

**第 25-27 行**：获取权重和像素值
```rust
let weight = mask[my as usize][mx as usize];
let pixel = src.get_pixel(sx, sy).0[0] as f32;
accum += pixel * weight;
```
- 从卷积核读取权重
- 从源图像读取像素值
- 累加：`accum += f(x,y) × h(i,j)`

**计算示例**（3×3 均值核）：
```
核：
[1, 1, 1]
[1, 1, 1]
[1, 1, 1]

邻域像素：
[100, 120, 110]
[130, 150, 140]
[120, 160, 130]

累加：
100×1 + 120×1 + 110×1 + 130×1 + 150×1 + 140×1 + 120×1 + 160×1 + 130×1
= 1260
```

**第 31-35 行**：归一化
```rust
let value = if weight_sum == 0.0 {
    accum
} else {
    accum / weight_sum
};
```
- 如果权重和为 0（边缘检测核），直接使用累加值
- 否则除以权重和进行归一化
- 示例（续）：`value = 1260 / 9 = 140`

**第 36 行**：限制范围并转换类型
```rust
let gray = value.round().clamp(0.0, 255.0) as u8;
```
- `.round()` 四舍五入
- `.clamp(0.0, 255.0)` 限制在合法灰度范围
- `as u8` 转换为字节

**第 37 行**：写入输出图像
```rust
dst.put_pixel(x as u32, y as u32, image::Luma([gray]));
```

---

### 第 40 行：返回结果
```rust
dst
```

## 常见卷积核示例

### 1. 均值滤波（3×3）
```rust
let mean_kernel = [[1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0]];
```
- 权重和：9
- 效果：平滑，去噪

### 2. 高斯滤波（3×3）
```rust
let gaussian_kernel = [[1.0, 2.0, 1.0],
                       [2.0, 4.0, 2.0],
                       [1.0, 2.0, 1.0]];
```
- 权重和：16
- 效果：平滑，保留边缘

### 3. 拉普拉斯算子（边缘检测）
```rust
let laplacian_kernel = [[0.0,  1.0, 0.0],
                        [1.0, -4.0, 1.0],
                        [0.0,  1.0, 0.0]];
```
- 权重和：0（不归一化）
- 效果：检测边缘

### 4. Sobel 算子（水平边缘）
```rust
let sobel_x = [[-1.0, 0.0, 1.0],
               [-2.0, 0.0, 2.0],
               [-1.0, 0.0, 1.0]];
```
- 权重和：0（不归一化）
- 效果：检测垂直边缘

## 应用场景

1. **图像平滑**：去除噪声
2. **边缘检测**：提取轮廓
3. **锐化**：增强细节
4. **模糊**：艺术效果

## 性能说明

- 时间复杂度：O(W×H × Kw×Kh)
  - `W×H` 是图像尺寸
  - `Kw×Kh` 是卷积核尺寸
- 空间复杂度：O(W×H)
- 对于 3×3 核，每像素 9 次乘加运算

## 使用示例

```rust
// 自定义锐化核
let sharpen_kernel = [[ 0.0, -1.0,  0.0],
                      [-1.0,  5.0, -1.0],
                      [ 0.0, -1.0,  0.0]];
let sharpened = img_filter(&gray_img, &sharpen_kernel);

// 5×5 强模糊
let blur_5x5 = [[1.0; 5]; 5];
let blurred = img_filter(&gray_img, &blur_5x5);
```
