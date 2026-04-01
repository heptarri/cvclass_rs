# img_median

## 函数签名

```rust
pub fn img_median(src: &GrayImage) -> GrayImage
```

## 功能描述

对图像进行中值滤波处理，使用 3×3 窗口内像素的中值替换中心像素。中值滤波是一种非线性滤波方法，特别适合去除椒盐噪声，同时能较好地保留边缘。

## 参数说明

- `src: &GrayImage` - 输入的灰度图像引用

## 返回值

- `GrayImage` - 中值滤波后的图像

## 数学原理

### 1. 中值定义

对于窗口内的像素集合 `{p₁, p₂, ..., pₙ}`，中值是排序后位于中间位置的值：

```
median({p₁, p₂, ..., pₙ}) = p₍ₙ₊₁₎/₂  (n 为奇数)
```

对于 3×3 窗口（9 个像素），中值是排序后的第 5 个元素（索引 4）。

### 2. 中值滤波公式

```
g(x, y) = median{f(x+i, y+j) | (i,j) ∈ W}
```

其中：
- `f(x, y)` 是输入图像
- `g(x, y)` 是输出图像
- `W` 是窗口（邻域）
- 3×3 窗口：`i, j ∈ {-1, 0, 1}`

### 3. 中值滤波的特性

**优点**：
1. **去除脉冲噪声**：椒盐噪声（极值）会被排序后的中间值替代
2. **保留边缘**：与均值滤波相比，不会模糊边缘
3. **非线性**：不会引入新的灰度值（只选择已有值）

**缺点**：
1. **计算量大**：需要排序操作
2. **细节损失**：小于窗口大小的细节可能被平滑掉
3. **角点退化**：锐角可能被钝化

### 4. 示例

输入 3×3 窗口：
```
[100, 120, 110]
[130, 250, 140]  ← 中心像素 250（可能是噪声）
[120, 160, 130]
```

排序：`[100, 110, 120, 120, 130, 130, 140, 160, 250]`

中值（第 5 个元素）：`130`

输出：中心像素从 250 变为 130（噪声被去除）

## 实现逻辑

### 第 140-142 行：初始化
```rust
let mut dst = GrayImage::new(src.width(), src.height());
let width = src.width() as i32;
let height = src.height() as i32;
```
**原理**：
- 创建输出图像
- 转换尺寸为有符号整数，方便处理边界

---

### 第 143-160 行：主循环 - 中值计算
```rust
for y in 0..height {
    for x in 0..width {
        let mut window = [0u8; 9];
        let mut idx = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let sx = (x + dx).clamp(0, width - 1) as u32;
                let sy = (y + dy).clamp(0, height - 1) as u32;
                let pixel = src.get_pixel(sx, sy).0[0];
                window[idx] = pixel;
                idx += 1;
            }
        }
        window.sort_unstable();
        dst.put_pixel(x as u32, y as u32, image::Luma([window[4]]));
    }
}
```
**原理**：

**第 143-144 行**：遍历图像所有像素

**第 145 行**：创建窗口数组
```rust
let mut window = [0u8; 9];
```
- 固定大小数组，存储 3×3 邻域的 9 个像素值
- 使用栈分配，高效

**第 146 行**：初始化索引
```rust
let mut idx = 0;
```
- 用于按顺序填充窗口数组

**第 147-154 行**：收集邻域像素
```rust
for dy in -1..=1 {  // Y 方向：-1, 0, 1
    for dx in -1..=1 {  // X 方向：-1, 0, 1
```
- 遍历 3×3 邻域的 9 个位置
- `..=` 表示包含右边界

**第 149-150 行**：计算采样坐标并边界处理
```rust
let sx = (x + dx).clamp(0, width - 1) as u32;
let sy = (y + dy).clamp(0, height - 1) as u32;
```
- 计算邻域坐标：`(x + dx, y + dy)`
- `.clamp()` 将坐标限制在图像边界内
- 边缘像素被复制（边缘复制策略）

**示例**：
- 当前像素 `(0, 0)` （左上角）
- `dx = -1` 时，`sx = clamp(0 - 1, 0, width-1) = 0`
- 左边界外的像素使用边界像素替代

**第 151-153 行**：填充窗口数组
```rust
let pixel = src.get_pixel(sx, sy).0[0];
window[idx] = pixel;
idx += 1;
```
- 读取像素值
- 按顺序存入窗口数组
- 索引递增

**窗口数组示例**：
```
window = [
    I(x-1,y-1), I(x,y-1), I(x+1,y-1),  // 上行
    I(x-1,y),   I(x,y),   I(x+1,y),    // 中行
    I(x-1,y+1), I(x,y+1), I(x+1,y+1)   // 下行
]
```

**第 156 行**：排序窗口数组
```rust
window.sort_unstable();
```
- `.sort_unstable()` 使用不稳定排序（更快）
- 对于 9 个元素，通常使用插入排序或小规模快速排序
- 时间复杂度：O(9 log 9) ≈ O(1) 对于固定小窗口

**排序示例**：
- 排序前：`[100, 120, 110, 130, 250, 140, 120, 160, 130]`
- 排序后：`[100, 110, 120, 120, 130, 130, 140, 160, 250]`

**第 157 行**：取中值并写入
```rust
dst.put_pixel(x as u32, y as u32, image::Luma([window[4]]));
```
- `window[4]` 是排序后的第 5 个元素（中值）
- 索引从 0 开始：`[0, 1, 2, 3, 4, 5, 6, 7, 8]`
- 中间位置：`(9 - 1) / 2 = 4`

---

### 第 161 行：返回结果
```rust
dst
```

## 与均值滤波的对比

| 特性 | 均值滤波 | 中值滤波 |
|------|---------|---------|
| 类型 | 线性 | 非线性 |
| 公式 | `mean({p})` | `median({p})` |
| 去噪能力（高斯噪声） | 好 | 一般 |
| 去噪能力（椒盐噪声） | 差 | **优秀** |
| 边缘保持 | 差（模糊） | **好** |
| 计算复杂度 | O(9) | O(9 log 9) |
| 适用场景 | 高斯噪声 | 脉冲噪声 |

## 应用场景

### 1. 椒盐噪声去除
```rust
// 图像中随机出现黑白点（0 或 255）
let noisy_img = add_salt_and_pepper(&clean_img, 0.05);
let denoised = img_median(&noisy_img);
```

### 2. 扫描文档处理
```rust
// 扫描文档常有脉冲噪声
let scanned_doc = load_scanned_document();
let cleaned = img_median(&scanned_doc);
```

### 3. 预处理
```rust
// 为边缘检测等后续操作准备
let filtered = img_median(&noisy_img);
let edges = img_sobel(&filtered);
```

### 4. 医学图像增强
```rust
// X 光、超声等医学图像常有脉冲噪声
let medical_img = load_medical_image();
let enhanced = img_median(&medical_img);
```

## 窗口大小的影响

本实现使用 3×3 窗口，可扩展为更大窗口：

- **3×3**：快速，适合轻度噪声
- **5×5**：中等强度，平衡效果和计算量
- **7×7**：强力去噪，但可能过度平滑

## 性能优化建议

### 1. 快速中值算法
- 对于大窗口，使用直方图方法
- 避免重复排序

### 2. 增量更新
- 滑动窗口时只更新变化的像素
- 减少排序开销

### 3. 近似中值
- 使用分位数估计代替精确排序
- 适合大窗口

## 使用示例

```rust
let gray_img = to_grayscale(&rgb_img);

// 添加椒盐噪声（模拟）
let mut noisy = gray_img.clone();
// ... 添加噪声代码 ...

// 应用中值滤波
let denoised = img_median(&noisy);
denoised.save("denoised.png")?;

// 多次应用（强力去噪）
let heavily_denoised = img_median(&img_median(&noisy));
```

## 性能说明

- 时间复杂度：O(W×H × 9 log 9) ≈ O(W×H)
  - 每像素：收集 9 个值 + 排序
- 空间复杂度：O(W×H + 9) ≈ O(W×H)
  - 输出图像 + 窗口数组（栈上）
- 相比均值滤波稍慢，但效果更好
