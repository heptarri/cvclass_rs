use std::path::Path;

use image::{ImageError, Rgb, RgbImage};

//滤波
// 平均平滑 高斯平滑 自适应平滑
// 中值滤波
// 锐化：Robert Sobel 拉普拉斯算子 高提升滤波 LoG

pub fn img_filter<const H: usize, const W: usize>(
    src: &RgbImage,
    mask: &[[f32; W]; H],
) -> RgbImage {
    let mut dst = RgbImage::new(src.width(), src.height());
    let width = src.width() as i32;
    let height = src.height() as i32;
    let half_w = (W / 2) as i32;
    let half_h = (H / 2) as i32;
    let weight_sum: f32 = mask.iter().flat_map(|row| row.iter()).copied().sum();

    for y in 0..height {
        for x in 0..width {
            let mut accum = [0.0f32; 3];
            for my in 0..H as i32 {
                for mx in 0..W as i32 {
                    let px = x + mx - half_w;
                    let py = y + my - half_h;
                    let sx = px.clamp(0, width - 1) as u32;
                    let sy = py.clamp(0, height - 1) as u32;
                    let weight = mask[my as usize][mx as usize];
                    let pixel = src.get_pixel(sx, sy).0;
                    for c in 0..3 {
                        accum[c] += pixel[c] as f32 * weight;
                    }
                }
            }

            let mut rgb = [0u8; 3];
            for c in 0..3 {
                let value = if weight_sum == 0.0 {
                    accum[c]
                } else {
                    accum[c] / weight_sum
                };
                rgb[c] = value.round().clamp(0.0, 255.0) as u8;
            }
            dst.put_pixel(x as u32, y as u32, Rgb(rgb));
        }
    }
    dst
}

/// 图像平滑均值
pub fn img_mean(src: &RgbImage) -> RgbImage {
    let kernel = &[[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
    img_filter(src, kernel)
}

pub fn run_chapt5(src: &RgbImage, output_root: &Path) -> Result<(), ImageError> {
    let meant = img_mean(src);
    meant.save(output_root.join("chapt5_meant.png"))?;
    println!("[TASK] mean img task completed!");
    Ok(())
}
