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

/// 最近邻插值缩放
pub fn img_resize_nearest(src: &RgbImage, w: u32, h: u32) -> RgbImage {
    img_resize(src, w, h)
}

/// 双线性插值缩放
pub fn img_resize_bilinear(src: &RgbImage, w: u32, h: u32) -> RgbImage {
    let mut dst = RgbImage::new(w, h);
    let sw = src.width() as f32;
    let sh = src.height() as f32;
    let max_x = src.width().saturating_sub(1) as i32;
    let max_y = src.height().saturating_sub(1) as i32;
    for y in 0..h {
        for x in 0..w {
            let fx = (x as f32 / w as f32) * sw;
            let fy = (y as f32 / h as f32) * sh;
            let x0 = fx.floor() as i32;
            let y0 = fy.floor() as i32;
            let x1 = (x0 + 1).clamp(0, max_x);
            let y1 = (y0 + 1).clamp(0, max_y);
            let x0 = x0.clamp(0, max_x);
            let y0 = y0.clamp(0, max_y);
            let dx = fx - x0 as f32;
            let dy = fy - y0 as f32;

            let p00 = src.get_pixel(x0 as u32, y0 as u32).0;
            let p10 = src.get_pixel(x1 as u32, y0 as u32).0;
            let p01 = src.get_pixel(x0 as u32, y1 as u32).0;
            let p11 = src.get_pixel(x1 as u32, y1 as u32).0;

            let mut rgb = [0u8; 3];
            for i in 0..3 {
                let v00 = p00[i] as f32;
                let v10 = p10[i] as f32;
                let v01 = p01[i] as f32;
                let v11 = p11[i] as f32;
                let v0 = v00 + (v10 - v00) * dx;
                let v1 = v01 + (v11 - v01) * dx;
                let v = v0 + (v1 - v0) * dy;
                rgb[i] = v.round().clamp(0.0, 255.0) as u8;
            }
            dst.put_pixel(x, y, image::Rgb(rgb));
        }
    }
    dst
}

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

/// 三次插值缩放（Catmull-Rom）
pub fn img_resize_cubic(src: &RgbImage, w: u32, h: u32) -> RgbImage {
    let mut dst = RgbImage::new(w, h);
    let sw = src.width() as f32;
    let sh = src.height() as f32;
    let max_x = src.width() as i32 - 1;
    let max_y = src.height() as i32 - 1;
    for y in 0..h {
        for x in 0..w {
            let fx = (x as f32 / w as f32) * sw;
            let fy = (y as f32 / h as f32) * sh;
            let ix = fx.floor() as i32;
            let iy = fy.floor() as i32;
            let dx = fx - ix as f32;
            let dy = fy - iy as f32;

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
            dst.put_pixel(x, y, image::Rgb(rgb));
        }
    }
    dst
}

/// 绕图像中心旋转指定角度，超出边界部分填充黑色。
pub fn img_rotate(src: &RgbImage, angle_deg: f32) -> RgbImage {
    img_rotate_cubic(src, angle_deg)
}

/// 最近邻插值旋转
pub fn img_rotate_nearest(src: &RgbImage, angle_deg: f32) -> RgbImage {
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

/// 双线性插值旋转
pub fn img_rotate_bilinear(src: &RgbImage, angle_deg: f32) -> RgbImage {
    let angle = angle_deg.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let width = src.width() as i32;
    let height = src.height() as i32;
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let max_x = src.width().saturating_sub(1) as i32;
    let max_y = src.height().saturating_sub(1) as i32;

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

            if src_x >= 0.0 && src_x < width as f32 && src_y >= 0.0 && src_y < height as f32 {
                let x0 = src_x.floor() as i32;
                let y0 = src_y.floor() as i32;
                let x1 = (x0 + 1).clamp(0, max_x);
                let y1 = (y0 + 1).clamp(0, max_y);
                let dx = src_x - x0 as f32;
                let dy = src_y - y0 as f32;

                let p00 = src.get_pixel(x0 as u32, y0 as u32).0;
                let p10 = src.get_pixel(x1 as u32, y0 as u32).0;
                let p01 = src.get_pixel(x0 as u32, y1 as u32).0;
                let p11 = src.get_pixel(x1 as u32, y1 as u32).0;

                let mut rgb = [0u8; 3];
                for i in 0..3 {
                    let v00 = p00[i] as f32;
                    let v10 = p10[i] as f32;
                    let v01 = p01[i] as f32;
                    let v11 = p11[i] as f32;
                    let v0 = v00 + (v10 - v00) * dx;
                    let v1 = v01 + (v11 - v01) * dx;
                    let v = v0 + (v1 - v0) * dy;
                    rgb[i] = v.round().clamp(0.0, 255.0) as u8;
                }
                dst.put_pixel(x as u32, y as u32, image::Rgb(rgb));
            }
        }
    }
    dst
}

/// 三次插值旋转（Catmull-Rom）
pub fn img_rotate_cubic(src: &RgbImage, angle_deg: f32) -> RgbImage {
    let angle = angle_deg.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let width = src.width() as i32;
    let height = src.height() as i32;
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let max_x = src.width() as i32 - 1;
    let max_y = src.height() as i32 - 1;

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
