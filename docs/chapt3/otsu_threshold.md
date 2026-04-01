# otsu_threshold

## 函数签名

```rust
pub fn otsu_threshold(gray: &GrayImage) -> u8
```

## 功能描述

使用 Otsu 算法自动计算图像的最佳二值化阈值。该算法通过最大化类间方差来寻找最优阈值，将图像分割为前景和背景两部分。

## 参数说明

- `gray: &GrayImage` - 输入的灰度图像引用

## 返回值

- `u8` - 最佳阈值，范围 [0, 255]

## 数学原理

Otsu 方法基于类间方差最大化原理：

### 1. 基本概念

假设用阈值 `t` 将图像分为两类：
- **类 0（背景）**：灰度值 ∈ [0, t]
- **类 1（前景）**：灰度值 ∈ [t+1, 255]

### 2. 统计量计算

对于给定阈值 `t`：

**类别权重**：
```
w₀(t) = Σ(i=0 to t) p(i)      # 背景像素比例
w₁(t) = Σ(i=t+1 to 255) p(i)  # 前景像素比例
```

**类别均值**：
```
μ₀(t) = Σ(i=0 to t) i·p(i) / w₀(t)      # 背景平均灰度
μ₁(t) = Σ(i=t+1 to 255) i·p(i) / w₁(t)  # 前景平均灰度
```

### 3. 类间方差

```
σ²ᵦ(t) = w₀(t) · w₁(t) · [μ₀(t) - μ₁(t)]²
```

**最优阈值**：
```
t* = argmax σ²ᵦ(t)
     t∈[0,255]
```

### 4. 优化计算

使用递推公式避免重复计算：
```
w₀(t+1) = w₀(t) + p(t+1)
μ₀(t+1) = [μ₀(t)·w₀(t) + (t+1)·p(t+1)] / w₀(t+1)
```

## 实现逻辑

### 第 89-95 行：计算直方图和基本统计量
```rust
let hist = calculate_histogram(gray);
let total: u32 = hist.iter().copied().sum();
let sum: f32 = hist
    .iter()
    .enumerate()
    .map(|(i, &count)| i as f32 * count as f32)
    .sum();
```
**原理**：

- **第 89 行**：计算灰度直方图
  - `hist[i]` = 灰度值为 `i` 的像素数量

- **第 90 行**：计算总像素数
  - `total = Σ hist[i]` = 图像总像素数

- **第 91-95 行**：计算加权灰度和
  - `sum = Σ(i × hist[i])`
  - 用于后续计算均值
  - 示例：如果 100 个像素灰度为 50，贡献 `50 × 100 = 5000`

---

### 第 97-101 行：初始化搜索变量
```rust
let mut sum_b = 0.0;
let mut w_b = 0u32;
let mut w_f;
let mut max_var = 0.0;
let mut threshold = 0u8;
```
**原理**：
- `sum_b`：背景类的加权灰度和
- `w_b`：背景类的像素数（权重）
- `w_f`：前景类的像素数
- `max_var`：记录最大类间方差
- `threshold`：记录最优阈值

---

### 第 103-122 行：遍历所有可能的阈值
```rust
for t in 0..256 {
    w_b += hist[t];
    if w_b == 0 {
        continue;
    }
    w_f = total - w_b;
    if w_f == 0 {
        break;
    }

    sum_b += (t as f32) * hist[t] as f32;
    let m_b = sum_b / w_b as f32;
    let m_f = (sum - sum_b) / w_f as f32;

    let var_between = (w_b as f32) * (w_f as f32) * (m_b - m_f).powi(2);
    if var_between > max_var {
        max_var = var_between;
        threshold = t as u8;
    }
}
```
**原理**：

- **第 103 行**：遍历所有可能的阈值 `t ∈ [0, 255]`

- **第 104-107 行**：更新背景类权重并跳过空类
  ```rust
  w_b += hist[t];
  if w_b == 0 { continue; }
  ```
  - 累加背景类像素数
  - 如果背景为空，跳过此阈值

- **第 108-111 行**：计算前景类权重并处理边界
  ```rust
  w_f = total - w_b;
  if w_f == 0 { break; }
  ```
  - 前景 = 总数 - 背景
  - 如果前景为空，后续阈值也会为空，直接退出

- **第 113-115 行**：计算两类的均值
  ```rust
  sum_b += (t as f32) * hist[t] as f32;
  let m_b = sum_b / w_b as f32;
  let m_f = (sum - sum_b) / w_f as f32;
  ```
  - **第 113 行**：递推更新背景加权和
  - **第 114 行**：背景均值 = 背景加权和 / 背景权重
  - **第 115 行**：前景均值 = 前景加权和 / 前景权重
  
  **示例计算**：
  - 假设 `t=100`, `w_b=5000`, `sum_b=250000`
  - 背景均值：`m_b = 250000 / 5000 = 50`
  - 前景均值：`m_f = (总和 - 250000) / (总数 - 5000)`

- **第 117 行**：计算类间方差
  ```rust
  let var_between = (w_b as f32) * (w_f as f32) * (m_b - m_f).powi(2);
  ```
  - 公式：`σ²ᵦ = w₀ · w₁ · (μ₀ - μ₁)²`
  - `.powi(2)` 计算平方（整数指数，比 `.powf(2.0)` 更快）
  
  **物理意义**：
  - 类间方差越大，说明前景和背景的灰度差异越明显
  - 最大化类间方差 = 最小化类内方差

- **第 118-121 行**：更新最优阈值
  ```rust
  if var_between > max_var {
      max_var = var_between;
      threshold = t as u8;
  }
  ```
  - 如果当前阈值的类间方差更大，更新最优值
  - 记录对应的阈值

---

### 第 124 行：返回最优阈值
```rust
threshold
```

## 算法特性

### 优点

1. **自适应**：无需手动指定阈值
2. **鲁棒性**：对光照变化有一定抵抗力
3. **全局最优**：遍历所有可能值，保证找到最大方差

### 适用条件

1. **双峰直方图**：图像有明显的前景和背景
2. **对比度充足**：前景和背景灰度差异明显
3. **噪声较少**：噪声会影响直方图分布

### 局限性

1. **多模态图像**：超过两个主要对象时效果不佳
2. **不均匀光照**：局部阈值可能更合适
3. **前景/背景比例极端**：一类占比过小时可能失效

## 使用示例

```rust
let gray_img = to_grayscale(&rgb_img);
let threshold = otsu_threshold(&gray_img);
println!("Otsu 最佳阈值: {}", threshold);

// 使用计算出的阈值进行二值化
let binary = gen_threshold(&rgb_img, threshold, 0);
```

## 性能说明

- 时间复杂度：O(256 + W×H) ≈ O(W×H)
  - 计算直方图：O(W×H)
  - 遍历阈值：O(256) = O(1)
- 空间复杂度：O(1) - 只需固定大小的数组和变量
- 非常高效，适合实时应用
