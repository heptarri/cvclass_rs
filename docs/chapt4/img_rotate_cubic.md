# img_rotate_cubic

## 函数签名

```rust
pub fn img_rotate_cubic(src: &RgbImage, angle_deg: f32) -> RgbImage
```

## 功能描述

使用三次（Bicubic）插值算法旋转图像。绕图像中心旋转指定角度，超出边界的部分填充黑色。Bicubic 插值提供最高质量的旋转效果，适合需要高质量输出的场景。

## 参数说明

- `src: &RgbImage` - 输入的 RGB 图像引用
- `angle_deg: f32` - 旋转角度（度数）
  - 正值：逆时针旋转
  - 负值：顺时针旋转
  - 示例：30.0 表示逆时针旋转 30 度

## 返回值

- `RgbImage` - 旋转后的图像，尺寸与原图相同

## 数学原理

### 1. 旋转变换矩阵

二维旋转的齐次变换矩阵：

```
[x']   [cos(θ)  -sin(θ)  0] [x - cx]
[y'] = [sin(θ)   cos(θ)  0] [y - cy]
[1 ]   [0        0       1] [1     ]
```

然后平移回中心：
```
x' = cos(θ)·(x - cx) - sin(θ)·(y - cy) + cx
y' = sin(θ)·(x - cx) + cos(θ)·(y - cy) + cy
```

其中：
- `(x, y)` 是目标图像坐标
- `(x', y')` 是源图像坐标
- `(cx, cy)` 是旋转中心（图像中心）
- `θ` 是旋转角度（弧度）

### 2. Bicubic 插值

对于非整数坐标 `(x', y')`，使用 Catmull-Rom 三次插值：

**权重函数**：
```
w(t) = {
    (a+2)|t|³ - (a+3)|t|² + 1          when |t| ≤ 1
    a|t|³ - 5a|t|² + 8a|t| - 4a        when 1 < |t| < 2
    0                                   when |t| ≥ 2
}
```
其中 `a = -0.5`（Catmull-Rom 参数）

**插值公式**：
```
f(x', y') = ΣΣ f(i, j) · w(x' - i) · w(y' - j)
            i j
```
求和范围：`i ∈ [⌊x'⌋-1, ⌊x'⌋+2]`, `j ∈ [⌊y'⌋-1, ⌊y'⌋+2]`

使用 4×4 邻域的 16 个像素进行插值。

## 实现逻辑

### 第 265-273 行：初始化旋转参数
```rust
let angle = angle_deg.to_radians();
let cos_a = angle.cos();
let sin_a = angle.sin();
let width = src.width() as i32;
let height = src.height() as i32;
let cx = width as f32 / 2.0;
let cy = height as f32 / 2.0;
let max_x = src.width() as i32 - 1;
let max_y = src.height() as i32 - 1;
```
**原理**：
- **第 265 行**：角度转换为弧度
  - `θ_rad = θ_deg × π / 180`
- **第 266-267 行**：预计算三角函数值
  - 避免在循环中重复计算
- **第 268-271 行**：图像尺寸和中心坐标
  - 中心：`(width/2, height/2)`
- **第 272-273 行**：边界坐标
  - 用于 clamp 操作

---

### 第 275-278 行：创建黑色背景
```rust
let mut dst = RgbImage::new(src.width(), src.height());
for pixel in dst.pixels_mut() {
    *pixel = image::Rgb([0, 0, 0]);
}
```
**原理**：
- 初始化输出图像，默认黑色
- 超出原图边界的区域保持黑色

---

### 第 280-314 行：主循环 - 旋转和插值
```rust
for y in 0..height {
    for x in 0..width {
        let xf = x as f32 - cx;
        let yf = y as f32 - cy;
        let src_x = cos_a * xf + sin_a * yf + cx;
        let src_y = -sin_a * xf + cos_a * yf + cy;

        if src_x >= 0.0 && src_x < width as f32 && src_y >= 0.0 && src_y < height as f32 {
            let ix = src_x.floor() as i32;
            let iy = src_y.floor() as i32;
            let dx = src_x - ix as f32;
            let dy = src_y - iy as f32;

            let mut rgb = [0.0f32; 3];
            for j in -1..=2 {
                let wy = cubic_weight(dy - j as f32);
                let sy = (iy + j).clamp(0, max_y);
                for i in -1..=2 {
                    let wx = cubic_weight(dx - i as f32);
                    let sx = (ix + i).clamp(0, max_x);
                    let pixel = src.get_pixel(sx as u32, sy as u32).0;
                    for c in 0..3 {
                        rgb[c] += pixel[c] as f32 * wx * wy;
                    }
                }
            }
            let rgb = [
                rgb[0].round().clamp(0.0, 255.0) as u8,
                rgb[1].round().clamp(0.0, 255.0) as u8,
                rgb[2].round().clamp(0.0, 255.0) as u8,
            ];
            dst.put_pixel(x as u32, y as u32, image::Rgb(rgb));
        }
    }
}
```
**原理**：

**第 280-281 行**：遍历目标图像的所有像素

**第 282-285 行**：计算逆向映射的源坐标
```rust
let xf = x as f32 - cx;
let yf = y as f32 - cy;
let src_x = cos_a * xf + sin_a * yf + cx;
let src_y = -sin_a * xf + cos_a * yf + cy;
```
- 将坐标平移到以中心为原点
- 应用旋转矩阵（注意是逆变换，角度取负）
- 平移回原坐标系

**示例**：
- 目标像素 `(400, 300)`, 中心 `(250, 250)`, 角度 30°
- 平移：`xf = 150, yf = 50`
- 旋转：
  ```
  src_x = cos(30°)×150 + sin(30°)×50 + 250 ≈ 404.9
  src_y = -sin(30°)×150 + cos(30°)×50 + 250 ≈ 218.3
  ```

**第 287 行**：边界检查
```rust
if src_x >= 0.0 && src_x < width as f32 && src_y >= 0.0 && src_y < height as f32 {
```
- 只处理映射到源图像内部的像素
- 否则保持黑色（初始值）

**第 288-291 行**：准备插值参数
```rust
let ix = src_x.floor() as i32;
let iy = src_y.floor() as i32;
let dx = src_x - ix as f32;
let dy = src_y - iy as f32;
```
- `(ix, iy)` 是整数部分（左上角像素）
- `(dx, dy)` 是小数部分，范围 [0, 1)
- 示例：`src_x = 404.7` → `ix = 404, dx = 0.7`

**第 293-305 行**：Bicubic 插值
```rust
let mut rgb = [0.0f32; 3];
for j in -1..=2 {  // Y方向：4个采样点
    let wy = cubic_weight(dy - j as f32);
    let sy = (iy + j).clamp(0, max_y);
    for i in -1..=2 {  // X方向：4个采样点
        let wx = cubic_weight(dx - i as f32);
        let sx = (ix + i).clamp(0, max_x);
        let pixel = src.get_pixel(sx as u32, sy as u32).0;
        for c in 0..3 {
            rgb[c] += pixel[c] as f32 * wx * wy;
        }
    }
}
```
- **4×4 邻域**：`[-1, 0, 1, 2]` 相对于 `(ix, iy)`
- **权重计算**：
  - X 方向：`wx = w(dx - i)`
  - Y 方向：`wy = w(dy - j)`
- **累加加权像素值**：
  - 对每个通道 `rgb[c] += pixel[c] × wx × wy`
- **边界处理**：`.clamp()` 确保不越界

**权重示例**（dx = 0.7）：
- `i=-1`: `w(0.7 - (-1)) = w(1.7)` ≈ -0.08
- `i=0`: `w(0.7 - 0) = w(0.7)` ≈ 0.49
- `i=1`: `w(0.7 - 1) = w(-0.3)` ≈ 0.71
- `i=2`: `w(0.7 - 2) = w(-1.3)` ≈ -0.12

**第 306-310 行**：限制范围并转换类型
```rust
let rgb = [
    rgb[0].round().clamp(0.0, 255.0) as u8,
    rgb[1].round().clamp(0.0, 255.0) as u8,
    rgb[2].round().clamp(0.0, 255.0) as u8,
];
```

**第 311 行**：写入目标像素
```rust
dst.put_pixel(x as u32, y as u32, image::Rgb(rgb));
```

---

### 第 315 行：返回结果
```rust
dst
```

## cubic_weight 函数（第 117-127 行）

```rust
fn cubic_weight(t: f32) -> f32 {
    let a = -0.5;
    let t = t.abs();
    if t <= 1.0 {
        (a + 2.0) * t.powi(3) - (a + 3.0) * t.powi(2) + 1.0
    } else if t < 2.0 {
        a * t.powi(3) - 5.0 * a * t.powi(2) + 8.0 * a * t - 4.0 * a
    } else {
        0.0
    }
}
```
**Catmull-Rom 三次样条**：
- `a = -0.5` 是标准参数
- 分段定义：
  - `|t| ≤ 1`: 中心区域，权重较大
  - `1 < |t| < 2`: 边缘区域，权重较小
  - `|t| ≥ 2`: 超出范围，权重为 0

## 插值方法对比

| 方法 | 邻域大小 | 质量 | 速度 | 适用场景 |
|------|----------|------|------|----------|
| 最近邻 | 1×1 | 低（锯齿明显） | 最快 | 实时预览 |
| 双线性 | 2×2 | 中（较平滑） | 快 | 一般应用 |
| 三次 | 4×4 | 高（最平滑） | 慢 | 高质量输出 |

## 使用示例

```rust
let img = image::open("photo.jpg")?.to_rgb8();

// 旋转 45 度
let rotated_45 = img_rotate_cubic(&img, 45.0);
rotated_45.save("rotated_45.jpg")?;

// 旋转 -30 度（顺时针 30 度）
let rotated_clockwise = img_rotate_cubic(&img, -30.0);
```

## 性能说明

- 时间复杂度：O(16 × 3 × W×H) = O(W×H)
  - 每像素处理 4×4 邻域
  - 3 个颜色通道
- 空间复杂度：O(W×H) - 输出图像
- 相比最近邻慢约 16 倍，但质量显著提升
