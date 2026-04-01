# run_chapt2

## 函数签名

```rust
pub fn run_chapt2(img: &RgbImage, output_root: &Path) -> Result<(), ImageError>
```

## 功能描述

`run_chapt2` 是 Chapter 2 的主函数，负责执行第二章的所有图像基本操作演示，包括：
- 获取图像信息
- 像素读取与写入
- 图像缩放
- 创建空白图像
- 灰度转换

## 参数说明

- `img: &RgbImage` - 输入的 RGB 彩色图像引用
- `output_root: &Path` - 输出文件保存的根目录路径

## 返回值

- `Result<(), ImageError>` - 成功返回 `Ok(())`，失败返回图像处理错误

## 实现逻辑

### 第 10 行：获取图像基本信息
```rust
get_info(img, 3);
```
**原理**：调用 `common::get_info()` 函数打印图像的宽度、高度和通道数（RGB 为 3 通道）。

---

### 第 11-13 行：读取指定像素值
```rust
if let Some(pixel) = get_pixel_value(img, 297, 260) {
    println!("Pixel value at (297,260): {:?}", pixel);
}
```
**原理**：
- 调用 `get_pixel_value()` 获取坐标 (row=297, col=260) 处的像素值
- 返回 `Option<[u8; 3]>`，如果坐标有效则返回 RGB 三通道值
- 使用 `if let` 模式匹配安全地处理可能的越界情况

---

### 第 15-22 行：像素写入操作
```rust
let mut modified = img.clone();
for y in 0..100u32.min(modified.height()) {
    for x in 0..100u32.min(modified.width()) {
        let _ = set_pixel_value(&mut modified, y, x, [0, 255, 255]);
    }
}
modified.save(output_root.join("chapt2_set_pixel.png"))?;
println!("[TASK] Set pixel task completed!");
```
**原理**：
- **第 15 行**：克隆原始图像，避免修改输入
- **第 16-20 行**：双重循环遍历图像左上角 100x100 区域
  - `100u32.min(modified.height())` 确保不超出图像边界
  - 将该区域所有像素设置为青色 `[R=0, G=255, B=255]`
- **第 21 行**：保存修改后的图像到 `chapt2_set_pixel.png`
- **数学原理**：直接像素赋值操作，`I(x,y) = [0, 255, 255]`，其中 `0 ≤ x,y < 100`

---

### 第 24-26 行：图像缩放
```rust
let resized = resize_image(img, 320, 240);
resized.save(output_root.join("chapt2_resized.png"))?;
println!("[TASK] Resize task completed!");
```
**原理**：
- 调用 `common::resize_image()` 将图像缩放到 320x240 分辨率
- 使用最近邻插值算法（详见 `common::resize_image()` 文档）
- **数学原理**：对于目标图像的每个像素 `(x', y')`，映射到源图像坐标：
  ```
  x_src = floor(x' / w_dst * w_src)
  y_src = floor(y' / h_dst * h_src)
  ```

---

### 第 28-30 行：创建空白图像
```rust
let blank = create_blank_image(320, 240, [0, 0, 255]);
blank.save(output_root.join("chapt2_blank.png"))?;
println!("[TASK] Create blank task completed!");
```
**原理**：
- 创建一个 320x240 的纯蓝色图像 `[R=0, G=0, B=255]`
- 所有像素值统一设置为指定颜色
- **数学表达**：`∀(x,y) ∈ [0,320)×[0,240), I(x,y) = [0,0,255]`

---

### 第 32-35 行：灰度转换
```rust
println!("Is grayscale: {}", is_grayscale(img));
let gray = to_grayscale(img);
gray.save(output_root.join("chapt2_gray.png"))?;
println!("[TASK] Change to gray task completed!");
```
**原理**：
- **第 32 行**：检查原图是否已经是灰度图（R=G=B）
- **第 33 行**：使用加权平均法转换为灰度图
- **数学公式**：
  ```
  Gray = 0.299 × R + 0.587 × G + 0.114 × B
  ```
  权重基于人眼对不同颜色的敏感度（绿色最敏感，蓝色最不敏感）

---

### 第 37 行：二值图检测
```rust
println!("Is binary: {}", is_binary(&gray));
```
**原理**：
- 检查灰度图是否为二值图像（仅包含 0 和 255 两种灰度值）
- 用于判断图像是否已经过二值化处理

---

### 第 39 行：返回结果
```rust
Ok(())
```
所有操作成功完成，返回空的成功结果。

## 输出文件

1. `chapt2_set_pixel.png` - 左上角 100x100 区域被设置为青色
2. `chapt2_resized.png` - 缩放到 320x240 的图像
3. `chapt2_blank.png` - 320x240 的纯蓝色图像
4. `chapt2_gray.png` - 灰度转换后的图像

## 使用示例

```rust
let img = image::open("input.png")?.to_rgb8();
let output_path = Path::new("./output");
run_chapt2(&img, &output_path)?;
```
