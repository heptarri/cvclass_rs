# cvcls_rs 函数文档

本目录包含 `cvcls_rs` 项目中所有图像处理函数的详细中文文档。每个函数都有独立的文档文件，包含数学原理、实现逻辑、代码逐行分析和使用示例。

## 文档结构

```
docs/
├── README.md           # 本文件
├── chapt2/             # 第二章：图像基本操作
├── chapt3/             # 第三章：灰度变换与直方图
├── chapt4/             # 第四章：几何变换
└── chapt5/             # 第五章：空间滤波与边缘检测
```

## 第二章：图像基本操作（chapt2）

### 已创建文档

- **run_chapt2.md** - 第二章主函数，演示所有基本操作

### 涵盖的核心操作（在 common.rs 中）

- `check_valid()` - 图像有效性检查
- `get_info()` - 获取图像基本信息（宽、高、通道数）
- `get_pixel_value()` - 读取指定坐标的像素值
- `set_pixel_value()` - 设置指定坐标的像素值
- `resize_image()` - 最近邻插值图像缩放
- `create_blank_image()` - 创建纯色图像
- `is_grayscale()` - 判断是否为灰度图
- `is_binary()` - 判断是否为二值图
- `to_grayscale()` - RGB 转灰度（加权平均法）

**主要内容**：像素级操作、图像信息获取、简单变换

---

## 第三章：灰度变换与直方图（chapt3）

### 已创建文档

1. **calculate_histogram.md** - 灰度直方图计算
2. **draw_histogram.md** - 直方图可视化
3. **gen_gray_lin_trans.md** - 线性灰度变换 `y = αx + β`
4. **gen_gray_log_trans.md** - 对数灰度变换 `y = c·log(1+x)`
5. **gen_gamma_trans.md** - Gamma 变换 `y = c·x^γ`
6. **otsu_threshold.md** - Otsu 自动阈值计算
7. **gen_equalize_hist.md** - 直方图均衡化

### 其他函数（可补充文档）

- `gen_threshold()` - 二值化变换（正向/反向/Otsu）
- `gen_piecewise_lin()` - 分段线性灰度映射
- `run_chapt3()` - 第三章主函数

**主要内容**：
- 灰度统计（直方图）
- 点运算（线性、对数、Gamma、分段线性）
- 图像分割（阈值化、Otsu 算法）
- 对比度增强（直方图均衡化）

---

## 第四章：几何变换（chapt4）

### 已创建文档

1. **pos_trans.md** - 图像平移变换
2. **img_rotate_cubic.md** - 三次插值图像旋转（高质量）

### 其他函数（可补充文档）

#### 基本几何变换
- `img_flip()` - 图像翻转（水平/垂直/双向）
- `img_transpose()` - 图像转置（行列互换）

#### 图像缩放
- `img_resize()` - 默认缩放（最近邻）
- `img_resize_nearest()` - 最近邻插值缩放
- `img_resize_bilinear()` - 双线性插值缩放
- `img_resize_cubic()` - 三次插值缩放（高质量）

#### 图像旋转
- `img_rotate()` - 默认旋转（三次插值）
- `img_rotate_nearest()` - 最近邻插值旋转
- `img_rotate_bilinear()` - 双线性插值旋转
- `img_rotate_cubic()` - 三次插值旋转（高质量）

#### 辅助函数
- `cubic_weight()` - Catmull-Rom 三次样条权重函数

**主要内容**：
- 刚体变换（平移、旋转、翻转、转置）
- 缩放变换
- 插值算法（最近邻、双线性、三次）
- 坐标映射与边界处理

---

## 第五章：空间滤波与边缘检测（chapt5）

### 已创建文档

1. **img_filter.md** - 通用空间域卷积滤波器
2. **img_sobel.md** - Sobel 边缘检测算子
3. **img_median.md** - 中值滤波（去椒盐噪声）

### 其他函数（可补充文档）

#### 平滑滤波
- `img_mean()` - 均值滤波（3×3）
- `img_gaussian()` - 高斯滤波（3×3）

#### 边缘检测
- `img_robert()` - Robert 算子（2×2，简单快速）
- `img_laplacian()` - 拉普拉斯算子（二阶导数）

#### 主函数
- `run_chapt5()` - 第五章主函数

**主要内容**：
- 卷积核设计
- 平滑滤波（去噪、模糊）
- 锐化滤波
- 边缘检测（一阶和二阶导数）
- 非线性滤波（中值）

---

## 如何使用文档

### 1. 快速查找

按章节浏览，每章对应一类图像处理操作：
- **基本操作** → chapt2
- **灰度处理** → chapt3
- **几何变换** → chapt4
- **滤波/边缘** → chapt5

### 2. 深入学习

每个函数文档包含：
- **函数签名**：参数和返回值类型
- **功能描述**：函数用途和特点
- **数学原理**：背后的理论公式
- **实现逻辑**：代码逐行解释
- **应用场景**：实际使用案例
- **使用示例**：完整代码示例
- **性能说明**：时间/空间复杂度

### 3. 代码导航

文档中的代码分析会引用源文件的行号，便于对照查看：
```rust
### 第 47 行：读取当前像素的灰度值
let value = gray.get_pixel(x, y)[0] as f32;
```

---

## 核心算法总结

### 点运算（Point Operations）
不依赖邻域，仅对单个像素操作：
- 线性变换：`y = αx + β`
- 对数变换：`y = c·log(1+x)`
- Gamma 变换：`y = c·x^γ`
- 查找表（LUT）：直方图均衡化、分段线性

### 邻域运算（Neighborhood Operations）
依赖像素邻域的操作：
- 卷积滤波：均值、高斯、Sobel、Laplacian
- 非线性滤波：中值、形态学
- 边缘检测：Robert、Sobel、拉普拉斯

### 几何变换（Geometric Transformations）
改变像素空间位置：
- 简单变换：平移、翻转、转置
- 仿射变换：旋转、缩放
- 插值方法：最近邻、双线性、三次

---

## 数学符号约定

文档中使用的常见数学符号：

- `f(x, y)` 或 `I(x, y)` - 输入图像
- `g(x, y)` - 输出图像
- `h(i, j)` - 卷积核（滤波器）
- `W × H` - 图像宽度和高度
- `[0, 255]` - 灰度值范围
- `∑` (Sigma) - 求和
- `∏` (Pi) - 求积
- `∂` (partial) - 偏导数
- `∇` (nabla) - 梯度

---

## 常见概念解释

### 卷积（Convolution）
将滤波器（卷积核）在图像上滑动，对每个位置进行加权求和。

### 插值（Interpolation）
在已知离散点之间估计连续值，用于图像缩放和旋转。

### 梯度（Gradient）
图像灰度变化的方向和强度，用于边缘检测。

### 直方图（Histogram）
统计每个灰度级出现的频次，反映图像的灰度分布。

### 阈值化（Thresholding）
将灰度图像转换为二值图像，用于分割前景和背景。

---

## 推荐学习路径

### 初学者
1. 从 **chapt2** 开始：理解像素操作和图像表示
2. 学习 **chapt3** 的线性变换：掌握点运算
3. 尝试 **chapt5** 的均值滤波：理解邻域运算

### 进阶学习
1. **chapt3** 的 Otsu 算法：学习自动阈值分割
2. **chapt3** 的直方图均衡化：理解图像增强
3. **chapt5** 的 Sobel 边缘检测：掌握梯度计算

### 高级应用
1. **chapt4** 的插值算法：理解重采样原理
2. **chapt4** 的旋转变换：掌握几何变换矩阵
3. **chapt5** 的通用滤波器：学习卷积核设计

---

## 参考资料

### 经典教材
- 冈萨雷斯《数字图像处理》（第3版）
- 胡学龙《数字图像处理》
- OpenCV 官方文档

### 在线资源
- [图像处理算法可视化](https://setosa.io/ev/image-kernels/)
- [Sobel 算子详解](https://en.wikipedia.org/wiki/Sobel_operator)
- [Catmull-Rom 样条](https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Catmull%E2%80%93Rom_spline)

---

## 文档维护

### 已完成
- ✅ chapt2: run_chapt2
- ✅ chapt3: 7 个核心函数
- ✅ chapt4: pos_trans, img_rotate_cubic
- ✅ chapt5: img_filter, img_sobel, img_median

### 待补充（可选）
- ⬜ chapt3: gen_threshold, gen_piecewise_lin, run_chapt3
- ⬜ chapt4: 其他插值和翻转函数
- ⬜ chapt5: img_mean, img_gaussian, img_robert, img_laplacian
- ⬜ common.rs: 基础工具函数

---

## 贡献指南

如需添加新的函数文档，请遵循以下结构：

```markdown
# 函数名

## 函数签名
## 功能描述
## 参数说明
## 返回值
## 数学原理
## 实现逻辑（逐行分析）
## 应用场景
## 使用示例
## 性能说明
```

确保：
1. 使用中文撰写
2. 包含数学公式和原理
3. 逐行解释关键代码
4. 提供实际使用示例
5. 说明时间/空间复杂度

---

## 联系方式

项目作者：[Heptari](https://heptari.uk/)  
课程：《图像处理与机器视觉》- 长安大学机器人工程系

---

**最后更新**：2026-04-01
