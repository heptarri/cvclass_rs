# img_sobel

## 函数签名

```rust
pub fn img_sobel(src: &GrayImage) -> GrayImage
```

## 功能描述

使用 Sobel 算子进行边缘检测，检测图像中的边缘和轮廓。Sobel 算子对噪声具有一定的抑制能力，是最常用的边缘检测方法之一。

## 参数说明

- `src: &GrayImage` - 输入的灰度图像引用

## 返回值

- `GrayImage` - 边缘检测后的图像，边缘处为高亮值，非边缘处为暗值

## 数学原理

### 1. Sobel 算子

Sobel 算子包含两个 3×3 卷积核，分别检测水平和垂直方向的梯度：

**水平方向（Gx）**：
```
     [-1  0  +1]
Gx = [-2  0  +2]
     [-1  0  +1]
```
检测垂直边缘（左右亮度变化）

**垂直方向（Gy）**：
```
     [-1  -2  -1]
Gy = [ 0   0   0]
     [+1  +2  +1]
```
检测水平边缘（上下亮度变化）

### 2. 梯度计算

对于图像中的每个像素 `(x, y)`：

```
Gx(x, y) = Σ Σ I(x+i, y+j) · Sx(i, j)
           i j

Gy(x, y) = Σ Σ I(x+i, y+j) · Sy(i, j)
           i j
```

其中 `I` 是输入图像，`Sx` 和 `Sy` 是 Sobel 核。

### 3. 梯度幅值

边缘强度（梯度幅值）计算：

**精确方法**（本实现使用）：
```
G(x, y) = √(Gx² + Gy²)
```

**近似方法**（更快但不太准确）：
```
G(x, y) = |Gx| + |Gy|
```

### 4. 梯度方向（可选）

```
θ(x, y) = arctan(Gy / Gx)
```
- `θ = 0°`: 水平边缘
- `θ = 90°`: 垂直边缘
- `θ = 45°/-45°`: 斜边缘

### 5. Sobel 的优势

- **平滑 + 求导**：结合了高斯平滑和中心差分
- **抗噪声**：中间行权重为 2，对噪声有抑制作用
- **方向敏感**：可以分别检测不同方向的边缘

## 实现逻辑

### 第 87-91 行：初始化
```rust
let mut dst = GrayImage::new(src.width(), src.height());
let width = src.width() as i32;
let height = src.height() as i32;
let gx = [[-1.0f32, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];
let gy = [[-1.0f32, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];
```
**原理**：
- 创建输出图像
- 定义 Sobel 卷积核
  - `gx`: 检测垂直边缘（X 方向梯度）
  - `gy`: 检测水平边缘（Y 方向梯度）

---

### 第 93-109 行：主循环 - 卷积和梯度计算
```rust
for y in 0..height {
    for x in 0..width {
        let mut acc_x = 0.0f32;
        let mut acc_y = 0.0f32;
        for dy in 0..3 {
            for dx in 0..3 {
                let sx = (x + dx - 1).clamp(0, width - 1) as u32;
                let sy = (y + dy - 1).clamp(0, height - 1) as u32;
                let pixel = src.get_pixel(sx, sy).0[0] as f32;
                acc_x += pixel * gx[dy as usize][dx as usize];
                acc_y += pixel * gy[dy as usize][dx as usize];
            }
        }
        let magnitude = (acc_x * acc_x + acc_y * acc_y).sqrt().clamp(0.0, 255.0) as u8;
        dst.put_pixel(x as u32, y as u32, Luma([magnitude]));
    }
}
```
**原理**：

**第 93-94 行**：遍历图像所有像素

**第 95-96 行**：初始化梯度累加器
```rust
let mut acc_x = 0.0f32;
let mut acc_y = 0.0f32;
```
- `acc_x` 存储 X 方向梯度
- `acc_y` 存储 Y 方向梯度

**第 97-104 行**：遍历 3×3 邻域并卷积
```rust
for dy in 0..3 {
    for dx in 0..3 {
```
- 遍历卷积核的每个位置

**第 99-100 行**：计算采样坐标并边界处理
```rust
let sx = (x + dx - 1).clamp(0, width - 1) as u32;
let sy = (y + dy - 1).clamp(0, height - 1) as u32;
```
- `x + dx - 1`: 将核中心对齐到当前像素
  - `dx=0` → `x-1` （左邻居）
  - `dx=1` → `x` （中心）
  - `dx=2` → `x+1` （右邻居）
- `.clamp()` 处理边界，边缘像素被复制

**第 101-103 行**：累加加权像素值
```rust
let pixel = src.get_pixel(sx, sy).0[0] as f32;
acc_x += pixel * gx[dy as usize][dx as usize];
acc_y += pixel * gy[dy as usize][dx as usize];
```
- 读取像素值
- 分别与 Gx 和 Gy 核相乘并累加

**计算示例**（假设 3×3 邻域）：
```
邻域像素值：
[100, 120, 140]
[110, 130, 150]
[120, 140, 160]

Gx 卷积：
= 100×(-1) + 120×0 + 140×1
+ 110×(-2) + 130×0 + 150×2
+ 120×(-1) + 140×0 + 160×1
= -100 + 0 + 140 - 220 + 0 + 300 - 120 + 0 + 160
= 160

Gy 卷积：
= 100×(-1) + 120×(-2) + 140×(-1)
+ 110×0 + 130×0 + 150×0
+ 120×1 + 140×2 + 160×1
= -100 - 240 - 140 + 0 + 0 + 0 + 120 + 280 + 160
= 80
```

**第 106 行**：计算梯度幅值
```rust
let magnitude = (acc_x * acc_x + acc_y * acc_y).sqrt().clamp(0.0, 255.0) as u8;
```
- 计算欧几里得距离：`√(Gx² + Gy²)`
- 示例（续）：`√(160² + 80²) = √(25600 + 6400) = √32000 ≈ 178.9`
- `.clamp(0.0, 255.0)` 限制在有效范围
- `as u8` 转换为字节

**物理意义**：
- 高幅值 → 强边缘
- 低幅值 → 平坦区域或弱边缘

**第 107 行**：写入结果
```rust
dst.put_pixel(x as u32, y as u32, Luma([magnitude]));
```

---

### 第 110 行：返回结果
```rust
dst
```

## 与其他边缘检测算子的对比

| 算子 | 核大小 | 抗噪能力 | 边缘定位 | 计算量 | 适用场景 |
|------|--------|----------|----------|--------|----------|
| Robert | 2×2 | 弱 | 精确 | 最小 | 低噪声图像 |
| Prewitt | 3×3 | 中 | 中等 | 中等 | 一般应用 |
| Sobel | 3×3 | 较强 | 中等 | 中等 | **最常用** |
| Laplacian | 3×3 | 弱 | 精确 | 小 | 二阶导数边缘 |
| Canny | 多阶段 | 最强 | 最精确 | 最大 | 高质量边缘 |

## Sobel 的典型应用

### 1. 边缘检测
```rust
let edges = img_sobel(&gray_img);
```

### 2. 特征提取
```rust
// 提取纹理特征
let texture_features = img_sobel(&gray_img);
```

### 3. 图像锐化
```rust
// 原图 + 边缘 = 锐化
// sharpened = original + α × sobel_edges
```

### 4. 预处理步骤
```rust
// 为轮廓检测、物体识别做准备
let preprocessed = img_sobel(&gray_img);
```

## 使用示例

```rust
let gray_img = to_grayscale(&rgb_img);
let edges = img_sobel(&gray_img);
edges.save("sobel_edges.png")?;

// 阈值化以获得清晰的边缘
let threshold = 50;
let binary_edges = gen_threshold(&edges, threshold, 0);
binary_edges.save("binary_edges.png")?;
```

## 性能说明

- 时间复杂度：O(9 × W×H) = O(W×H)
  - 每像素 18 次乘法（2 个 3×3 卷积）
  - 1 次平方根运算
- 空间复杂度：O(W×H)
- 相对高效，适合实时应用

## 改进方向

1. **可分离卷积**：
   - Sobel 可分解为 1D 卷积的组合
   - 减少计算量：9 次乘法 → 6 次乘法

2. **自适应阈值**：
   - 对边缘幅值进行阈值化
   - 抑制弱边缘和噪声

3. **非极大值抑制**：
   - 保留局部最大的边缘响应
   - 得到更细的边缘线（Canny 算法的一部分）
