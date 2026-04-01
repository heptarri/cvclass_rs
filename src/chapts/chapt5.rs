use std::path::Path;

use image::{GrayImage, ImageError};

//滤波
// 平均平滑 高斯平滑 自适应平滑
// 中值滤波
// 锐化：Robert Sobel 拉普拉斯算子 高提升滤波 LoG

pub fn img_filter<const H: usize, const W: usize>(
    src: &GrayImage,
    mask: &[[f32; W]; H],
) -> GrayImage {
    let mut dst = GrayImage::new(src.width(), src.height());
    let width = src.width() as i32;
    let height = src.height() as i32;
    let half_w = (W / 2) as i32;
    let half_h = (H / 2) as i32;
    let weight_sum: f32 = mask.iter().flat_map(|row| row.iter()).copied().sum();

    for y in 0..height {
        for x in 0..width {
            let mut accum = 0.0f32;
            for my in 0..H as i32 {
                for mx in 0..W as i32 {
                    let px = x + mx - half_w;
                    let py = y + my - half_h;
                    let sx = px.clamp(0, width - 1) as u32;
                    let sy = py.clamp(0, height - 1) as u32;
                    let weight = mask[my as usize][mx as usize];
                    let pixel = src.get_pixel(sx, sy).0[0] as f32;
                    accum += pixel * weight;
                }
            }

            let value = if weight_sum == 0.0 {
                accum
            } else {
                accum / weight_sum
            };
            let gray = value.round().clamp(0.0, 255.0) as u8;
            dst.put_pixel(x as u32, y as u32, image::Luma([gray]));
        }
    }
    dst
}

/// 均值平滑
pub fn img_mean(src: &GrayImage) -> GrayImage {
    let kernel = &[[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
    img_filter(src, kernel)
}

/// 高斯平滑
pub fn img_gaussian(src: &GrayImage) -> GrayImage {
    let kernel = &[[1.0, 2.0, 1.0], [2.0, 4.0, 2.0], [1.0, 2.0, 1.0]];
    img_filter(src, kernel)
}

/// 中值滤波（3x3）
pub fn img_median(src: &GrayImage) -> GrayImage {
    let mut dst = GrayImage::new(src.width(), src.height());
    let width = src.width() as i32;
    let height = src.height() as i32;
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
    dst
}

pub fn run_chapt5(src: &GrayImage, output_root: &Path) -> Result<(), ImageError> {
    let meant = img_mean(src);
    meant.save(output_root.join("chapt5_meant.png"))?;
    println!("[TASK] mean img task completed!");

    let gaussian = img_gaussian(src);
    gaussian.save(output_root.join("chapt5_gauss.png"))?;
    println!("[TASK] gaussian img task completed!");

    let median = img_median(src);
    median.save(output_root.join("chapt5_median.png"))?;
    println!("[TASK] median img task completed!");

    Ok(())
}
