use image::{ImageError, RgbImage};
use std::path::Path;

/// 图像平移变换，超出边界处填充黑色。
pub fn pos_trans(src: &RgbImage, tx: i32, ty: i32) -> RgbImage {
    let mut dst = RgbImage::new(src.width(), src.height());
    for pixel in dst.pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }
    for y in 0..src.height() {
        for x in 0..src.width() {
            let target_x = x as i32 + tx;
            let target_y = y as i32 + ty;
            if target_x >= 0
                && target_x < src.width() as i32
                && target_y >= 0
                && target_y < src.height() as i32
            {
                dst.put_pixel(target_x as u32, target_y as u32, *src.get_pixel(x, y));
            }
        }
    }
    dst
}

/// 对图像进行翻转：0=垂直，1=水平，-1=水平+垂直。
pub fn img_flip(src: &RgbImage, mode: i8) -> RgbImage {
    let mut dst = RgbImage::new(src.width(), src.height());
    for y in 0..src.height() {
        for x in 0..src.width() {
            let target = match mode {
                0 => (x, src.height() - 1 - y),
                -1 => (src.width() - 1 - x, src.height() - 1 - y),
                _ => (src.width() - 1 - x, y),
            };
            dst.put_pixel(target.0, target.1, *src.get_pixel(x, y));
        }
    }
    dst
}

/// 转置图像，行列互换。
pub fn img_transpose(src: &RgbImage) -> RgbImage {
    let mut dst = RgbImage::new(src.height(), src.width());
    for y in 0..src.height() {
        for x in 0..src.width() {
            dst.put_pixel(y, x, *src.get_pixel(x, y));
        }
    }
    dst
}

/// 使用最近邻插值缩放图像。
pub fn img_resize(src: &RgbImage, w: u32, h: u32) -> RgbImage {
    let mut dst = RgbImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let src_x = ((x as f32 / w as f32) * src.width() as f32).floor() as u32;
            let src_y = ((y as f32 / h as f32) * src.height() as f32).floor() as u32;
            let src_x = src_x.min(src.width().saturating_sub(1));
            let src_y = src_y.min(src.height().saturating_sub(1));
            let pixel = src.get_pixel(src_x, src_y);
            dst.put_pixel(x, y, *pixel);
        }
    }
    dst
}

/// 绕图像中心旋转指定角度，超出边界部分填充黑色。
pub fn img_rotate(src: &RgbImage, angle_deg: f32) -> RgbImage {
    let angle = angle_deg.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let width = src.width() as i32;
    let height = src.height() as i32;
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;

    let mut dst = RgbImage::new(src.width(), src.height());
    for pixel in dst.pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }

    for y in 0..height {
        for x in 0..width {
            let xf = x as f32 - cx;
            let yf = y as f32 - cy;
            let src_x = cos_a * xf + sin_a * yf + cx;
            let src_y = -sin_a * xf + cos_a * yf + cy;
            let sx = src_x.round() as i32;
            let sy = src_y.round() as i32;
            if sx >= 0 && sx < width && sy >= 0 && sy < height {
                dst.put_pixel(x as u32, y as u32, *src.get_pixel(sx as u32, sy as u32));
            }
        }
    }
    dst
}

/// 运行 Chapter 4 的图像处理流程，并保存输出文件。
pub fn run_chapt4(img: &RgbImage, output_root: &Path) -> Result<(), ImageError> {
    let translated = pos_trans(img, 100, 100);
    translated.save(output_root.join("chapt4_translated.png"))?;

    let flipped = img_flip(img, 1);
    flipped.save(output_root.join("chapt4_flipped.png"))?;

    let transposed = img_transpose(img);
    transposed.save(output_root.join("chapt4_transposed.png"))?;

    let resized = img_resize(img, 100, 100);
    resized.save(output_root.join("chapt4_resized.png"))?;

    let rotated = img_rotate(img, 30.0);
    rotated.save(output_root.join("chapt4_rotated.png"))?;
    Ok(())
}
