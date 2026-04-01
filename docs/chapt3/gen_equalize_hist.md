# gen_equalize_hist

## 函数签名

```rust
pub fn gen_equalize_hist(src: &RgbImage) -> GrayImage
```

## 功能描述

对图像进行直方图均衡化处理，使灰度分布更加均匀，从而增强图像的整体对比度。该方法通过累积分布函数（CDF）重新映射像素值。

## 参数说明

- `src: &RgbImage` - 输入的 RGB 彩色图像引用

## 返回值

- `GrayImage` - 直方图均衡化后的灰度图像

## 数学原理

### 1. 概率密度函数（PDF）

```
p(rₖ) = nₖ / n
```
- `nₖ` 是灰度级 `rₖ` 的像素数
- `n` 是图像总像素数
- `p(rₖ)` 是灰度级 `rₖ` 的概率

### 2. 累积分布函数（CDF）

```
cdf(k) = Σ(i=0 to k) p(rᵢ) = Σ(i=0 to k) nᵢ / n
```

### 3. 均衡化映射公式

```
sₖ = floor[(L-1) × (cdf(k) - cdf_min) / (1 - cdf_min)]
```

其中：
- `L = 256`（灰度级总数）
- `cdf_min` 是第一个非零的 CDF 值
- 分母 `(1 - cdf_min)` 用于归一化

**简化为**：
```
sₖ = 255 × (cdf(k) - cdf_min) / (n - cdf_min)
```

### 4. 原理解释

- **目标**：使变换后的直方图尽可能均匀分布
- **方法**：将累积分布拉伸到整个灰度范围
- **效果**：高频灰度级被分散，低频灰度级被保留

## 实现逻辑

### 第 193-195 行：计算直方图和CDF
```rust
let gray = to_grayscale(src);
let hist = calculate_histogram(&gray);
let total = gray.width() * gray.height();
```
**原理**：
- 转换为灰度图
- 计算直方图：`hist[i]` = 灰度 `i` 的像素数
- 计算总像素数：用于归一化

---

### 第 196-201 行：构建累积分布函数
```rust
let mut cdf = [0u32; 256];
let mut cumulative = 0u32;
for i in 0..256 {
    cumulative += hist[i];
    cdf[i] = cumulative;
}
```
**原理**：
- **第 196 行**：初始化 CDF 数组
- **第 197 行**：累加器，初始为 0
- **第 198-201 行**：递推计算 CDF
  - `cdf[i] = Σ(k=0 to i) hist[k]`
  - 累积到灰度级 `i` 的总像素数
  
**示例**：
- `hist = [50, 100, 80, ...]`
- `cdf[0] = 50`
- `cdf[1] = 50 + 100 = 150`
- `cdf[2] = 150 + 80 = 230`
- `...`

---

### 第 202-203 行：找到最小非零CDF
```rust
let cdf_min = cdf.iter().copied().find(|&v| v > 0).unwrap_or(0) as f32;
let total_f = total as f32;
```
**原理**：
- **第 202 行**：
  - `.find(|&v| v > 0)` 找到第一个大于 0 的 CDF 值
  - 对应最小的有效灰度级
  - `.unwrap_or(0)` 处理全黑图像的情况
- **第 203 行**：将总像素数转换为浮点数，用于除法运算

---

### 第 205-210 行：构建查找表（LUT）
```rust
let mut lut = [0u8; 256];
for i in 0..256 {
    let numerator = cdf[i] as f32 - cdf_min;
    let mapped = ((numerator / (total_f - cdf_min)) * 255.0).round();
    lut[i] = mapped.clamp(0.0, 255.0) as u8;
}
```
**原理**：

- **第 205 行**：初始化查找表，将旧灰度映射到新灰度

- **第 206 行**：遍历所有灰度级

- **第 207 行**：计算分子
  ```rust
  let numerator = cdf[i] as f32 - cdf_min;
  ```
  - 从 CDF 中减去最小值，实现归一化

- **第 208 行**：应用均衡化公式
  ```rust
  let mapped = ((numerator / (total_f - cdf_min)) * 255.0).round();
  ```
  - 分母：`total - cdf_min` = 实际有效像素数
  - 除法：归一化到 [0, 1]
  - 乘以 255：映射到 [0, 255]
  - `.round()` 四舍五入
  
  **数学表达**：
  ```
  lut[i] = 255 × (cdf[i] - cdf_min) / (n - cdf_min)
  ```
  
  **示例计算**：
  - 假设 `cdf[100] = 5000`, `cdf_min = 100`, `total = 10000`
  - `numerator = 5000 - 100 = 4900`
  - `mapped = (4900 / 9900) × 255 ≈ 126`
  - 旧灰度 100 → 新灰度 126

- **第 209 行**：限制范围并转换类型
  ```rust
  lut[i] = mapped.clamp(0.0, 255.0) as u8;
  ```

---

### 第 212-218 行：应用查找表
```rust
let mut dst = GrayImage::new(gray.width(), gray.height());
for y in 0..gray.height() {
    for x in 0..gray.width() {
        let value = gray.get_pixel(x, y)[0];
        dst.put_pixel(x, y, Luma([lut[value as usize]]));
    }
}
```
**原理**：
- 遍历所有像素
- 读取旧灰度值 `value`
- 通过查找表映射到新灰度值 `lut[value]`
- 写入目标图像

**时间优化**：
- 查找表方法避免对每个像素重复计算
- 时间复杂度：O(W×H)，每像素仅一次查表

---

### 第 219 行：返回结果
```rust
dst
```

## 应用场景

### 1. 低对比度图像增强
```rust
// 雾天、烟雾导致的低对比度
let enhanced = gen_equalize_hist(&foggy_img);
```

### 2. 医学影像
```rust
// X 光片、CT 扫描的对比度增强
let medical_enhanced = gen_equalize_hist(&xray_img);
```

### 3. 视频监控
```rust
// 夜间监控画面增强
let surveillance_enhanced = gen_equalize_hist(&night_video_frame);
```

## 效果分析

**优点**：
- 自动增强对比度
- 突出细节
- 无需手动参数

**局限性**：
1. **过度增强**：可能放大噪声
2. **不自然**：某些区域可能过亮或过暗
3. **全局方法**：无法处理局部光照不均

**改进方法**：
- **CLAHE**（Contrast Limited Adaptive Histogram Equalization）
- 对图像分块，分别均衡化
- 限制对比度增强的幅度

## 使用示例

```rust
let rgb_img = image::open("low_contrast.jpg")?.to_rgb8();
let equalized = gen_equalize_hist(&rgb_img);
equalized.save("high_contrast.jpg")?;

// 对比原始直方图和均衡化后的直方图
let orig_hist = calculate_histogram(&to_grayscale(&rgb_img));
let eq_hist = calculate_histogram(&equalized);
```

## 性能说明

- 时间复杂度：O(W×H + 256) ≈ O(W×H)
  - 计算直方图：O(W×H)
  - 计算 CDF：O(256)
  - 应用 LUT：O(W×H)
- 空间复杂度：O(W×H + 512) - 输出图像 + 直方图/CDF/LUT
- 高效且实用
