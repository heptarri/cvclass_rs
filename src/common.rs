use image::{GenericImageView, GrayImage, Luma, Rgb, RgbImage};

/// 检查图像是否有效：宽度、高度大于 0。
pub fn check_valid<I: GenericImageView>(img: &I) -> bool {
    img.width() > 0 && img.height() > 0
}

/// 输出图像基本信息，包括宽、高和通道数。
pub fn get_info<I: GenericImageView>(img: &I, channels: usize) {
    println!("Basic information:");
    println!("Width: {} px", img.width());
    println!("Height: {} px", img.height());
    println!("Channels: {}", channels);
}

/// 获取指定像素位置的值。返回一个 RGB 三元组。
pub fn get_pixel_value(img: &RgbImage, row: u32, col: u32) -> Option<[u8; 3]> {
    if row >= img.height() || col >= img.width() {
        return None;
    }
    Some(img.get_pixel(col, row).0)
}

/// 在指定位置写入像素值。
pub fn set_pixel_value(img: &mut RgbImage, row: u32, col: u32, value: [u8; 3]) -> bool {
    if row >= img.height() || col >= img.width() {
        return false;
    }
    img.put_pixel(col, row, Rgb(value));
    true
}

/// 最近邻缩放：将图像缩放到指定宽高。
pub fn resize_image(src: &RgbImage, width: u32, height: u32) -> RgbImage {
    let mut dst = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let src_x = ((x as f32 / width as f32) * src.width() as f32).floor() as u32;
            let src_y = ((y as f32 / height as f32) * src.height() as f32).floor() as u32;
            let src_x = src_x.min(src.width().saturating_sub(1));
            let src_y = src_y.min(src.height().saturating_sub(1));
            let pixel = src.get_pixel(src_x, src_y);
            dst.put_pixel(x, y, *pixel);
        }
    }
    dst
}

/// 创建指定尺寸的纯色图像。
pub fn create_blank_image(width: u32, height: u32, color: [u8; 3]) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    for pixel in img.pixels_mut() {
        *pixel = Rgb(color);
    }
    img
}

/// 判断彩色图像是否为灰度图（R == G == B 对所有像素成立）。
pub fn is_grayscale(img: &RgbImage) -> bool {
    img.pixels().all(|p| {
        let [r, g, b] = p.0;
        r == g && g == b
    })
}

/// 判断灰度图像是否为二值图像（仅包含 0 和 255 两种灰度值）。
pub fn is_binary(img: &GrayImage) -> bool {
    img.pixels().all(|p| {
        let v = p[0];
        v == 0 || v == 255
    })
}

/// 将彩色图像转换为灰度图像，使用加权平均法。
pub fn to_grayscale(src: &RgbImage) -> GrayImage {
    let mut gray = GrayImage::new(src.width(), src.height());
    for y in 0..src.height() {
        for x in 0..src.width() {
            let [r, g, b] = src.get_pixel(x, y).0;
            let value = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32).round();
            gray.put_pixel(x, y, Luma([value.min(255.0) as u8]));
        }
    }
    gray
}
