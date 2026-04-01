# gen_gamma_trans

## 函数签名

```rust
pub fn gen_gamma_trans(src: &RgbImage, gamma: f32) -> RgbImage
```

## 功能描述

对图像进行 Gamma 校正（幂律变换），使用公式 `s = c · r^γ` 调整图像的亮度和对比度。Gamma 变换广泛应用于显示器校正和图像增强。

## 参数说明

- `src: &RgbImage` - 输入的 RGB 彩色图像引用
- `gamma: f32` - Gamma 值（幂指数）
  - `γ > 1`：压缩低灰度值，图像变暗
  - `γ = 1`：不变换（线性映射）
  - `γ < 1`：扩展低灰度值，图像变亮

## 返回值

- `RgbImage` - Gamma 变换后的 RGB 图像（保留颜色信息）

## 数学原理

Gamma 变换的标准公式：

```
s = 255 · (r / 255)^γ
```

其中：
- `r` 是输入像素值，范围 [0, 255]
- `s` 是输出像素值
- `γ` 是 Gamma 参数
- 归一化到 [0, 1] 后进行幂运算，再缩放回 [0, 255]

### Gamma 值的影响

**当 γ < 1（例如 γ = 0.5）**：
- 曲线向上凸
- 暗部被提亮（扩展）
- 亮部变化较小
- 示例：输入 50 → 输出 ~112

**当 γ = 1**：
- 线性映射，输出 = 输入

**当 γ > 1（例如 γ = 2.5）**：
- 曲线向下凹
- 暗部被进一步压暗
- 亮部保持较亮
- 示例：输入 50 → 输出 ~4

### 映射示例（γ = 2.5）

| 输入 | 归一化 | 幂运算 | 输出 |
|------|--------|--------|------|
| 0    | 0.000  | 0.000  | 0    |
| 50   | 0.196  | 0.016  | 4    |
| 100  | 0.392  | 0.091  | 23   |
| 150  | 0.588  | 0.268  | 68   |
| 200  | 0.784  | 0.563  | 144  |
| 255  | 1.000  | 1.000  | 255  |

## 实现逻辑

### 第 72 行：创建目标图像
```rust
let mut dst = RgbImage::new(src.width(), src.height());
```
**原理**：创建与输入相同尺寸的空白 RGB 图像。

---

### 第 73-83 行：遍历并应用 Gamma 变换
```rust
for y in 0..src.height() {
    for x in 0..src.width() {
        let [r, g, b] = src.get_pixel(x, y).0;
        let convert = |value: u8| {
            let normalized = value as f32 / 255.0;
            let mapped = normalized.powf(gamma) * 255.0;
            mapped.clamp(0.0, 255.0) as u8
        };
        dst.put_pixel(x, y, Rgb([convert(r), convert(g), convert(b)]));
    }
}
```
**原理**：

- **第 73-74 行**：双重循环遍历所有像素坐标

- **第 75 行**：解构获取 RGB 三个通道的值
  - `.0` 提取 `Rgb` 元组的内部数组
  - `[r, g, b]` 模式匹配三个通道

- **第 76-80 行**：定义转换闭包
  - 闭包捕获外部的 `gamma` 变量
  - 对单个通道值进行 Gamma 变换
  
  **第 77 行**：归一化到 [0, 1]
  ```rust
  let normalized = value as f32 / 255.0;
  ```
  - 将 [0, 255] 映射到 [0.0, 1.0]
  - 示例：100 / 255 ≈ 0.392
  
  **第 78 行**：应用幂运算并缩放回 [0, 255]
  ```rust
  let mapped = normalized.powf(gamma) * 255.0;
  ```
  - `.powf(gamma)` 计算 `normalized^γ`
  - 示例：`0.392^2.5 ≈ 0.091`
  - 乘以 255：`0.091 × 255 ≈ 23.2`
  
  **第 79 行**：限制范围并转换类型
  ```rust
  mapped.clamp(0.0, 255.0) as u8
  ```
  - 确保在 [0, 255] 范围内
  - 转换为字节类型

- **第 81 行**：对三个通道分别应用变换
  ```rust
  dst.put_pixel(x, y, Rgb([convert(r), convert(g), convert(b)]));
  ```
  - 分别处理 R、G、B 通道
  - 保持颜色信息（不转灰度）

---

### 第 84 行：返回结果
```rust
dst
```

## 应用场景

### 1. 显示器 Gamma 校正
```rust
// CRT 显示器的 gamma 通常是 2.2，需要补偿
let corrected = gen_gamma_trans(&img, 1.0 / 2.2); // γ ≈ 0.45
```

### 2. 图像增强
```rust
// 提亮暗部细节（适用于欠曝图像）
let brightened = gen_gamma_trans(&img, 0.5);

// 增强对比度（适用于过曝图像）
let darkened = gen_gamma_trans(&img, 2.5);
```

### 3. HDR 压缩
```rust
// 压缩高动态范围图像到标准显示范围
let compressed = gen_gamma_trans(&hdr_img, 2.2);
```

## 与其他变换的对比

| 变换类型 | 公式 | 特点 |
|---------|------|------|
| 线性 | `y = αx + β` | 均匀拉伸/压缩 |
| 对数 | `y = c·ln(1+x)` | 扩展暗部，压缩亮部 |
| Gamma | `y = c·x^γ` | 灵活调整中间调 |
| 指数 | `y = c·(e^x - 1)` | 扩展亮部，压缩暗部 |

## 标准 Gamma 值

- **γ = 0.4-0.5**：图像采集（相机编码）
- **γ = 2.2**：sRGB 标准显示
- **γ = 2.4**：Rec. 709（HDTV 标准）
- **γ = 2.6**：DCI-P3（数字影院）

## 使用示例

```rust
// 示例 1：校正欠曝图像
let underexposed = image::open("dark.jpg")?.to_rgb8();
let corrected = gen_gamma_trans(&underexposed, 0.5);
corrected.save("corrected.jpg")?;

// 示例 2：创建夜间效果
let daytime = image::open("day.jpg")?.to_rgb8();
let night_effect = gen_gamma_trans(&daytime, 3.0);
night_effect.save("night.jpg")?;
```

## 性能说明

- 时间复杂度：O(3 × W × H) - 处理三个通道
- 每像素操作：3 次幂运算（R、G、B）
- `.powf()` 相对耗时，但对于图像处理仍可接受
