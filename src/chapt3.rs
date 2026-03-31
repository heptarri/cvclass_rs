use crate::common::to_grayscale;
use image::{GrayImage, ImageError, Luma, Rgb, RgbImage};
use std::path::Path;

/// 计算灰度直方图，并返回 256 个灰度级的频次。
pub fn calculate_histogram(gray: &GrayImage) -> [u32; 256] {
    let mut hist = [0u32; 256];
    for pixel in gray.pixels() {
        hist[pixel[0] as usize] += 1;
    }
    hist
}

/// 将直方图绘制为图像，白色柱状图，黑色背景。
pub fn draw_histogram(hist: &[u32; 256]) -> RgbImage {
    let hist_h = 400;
    let hist_w = 512;
    let mut img = RgbImage::new(hist_w, hist_h);
    for pixel in img.pixels_mut() {
        *pixel = Rgb([0, 0, 0]);
    }

    let max_count = hist.iter().copied().max().unwrap_or(1) as f32;
    let bin_w = hist_w / 256;
    let bin_w = bin_w.max(1);

    for (i, &count) in hist.iter().enumerate() {
        let normalized = ((count as f32 / max_count) * (hist_h as f32 - 1.0)).round() as u32;
        let x0 = i as u32 * bin_w;
        let x1 = (i as u32 + 1) * bin_w;
        let x1 = x1.min(hist_w);
        for x in x0..x1 {
            for y in (hist_h - normalized)..hist_h {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }
    img
}

/// 线性灰度变换：y = alpha * x + beta。
pub fn gen_gray_lin_trans(src: &RgbImage, alpha: f32, beta: f32) -> GrayImage {
    let gray = to_grayscale(src);
    let mut dst = GrayImage::new(gray.width(), gray.height());
    for y in 0..gray.height() {
        for x in 0..gray.width() {
            let value = gray.get_pixel(x, y)[0] as f32;
            let mapped = (alpha * value + beta).round();
            dst.put_pixel(x, y, Luma([mapped.clamp(0.0, 255.0) as u8]));
        }
    }
    dst
}

/// 对数灰度变换：y = log(1 + r) * (255 / log(256))。
pub fn gen_gray_log_trans(src: &RgbImage) -> GrayImage {
    let gray = to_grayscale(src);
    let scale = 255.0 / 256_f32.ln();
    let mut dst = GrayImage::new(gray.width(), gray.height());
    for y in 0..gray.height() {
        for x in 0..gray.width() {
            let value = gray.get_pixel(x, y)[0] as f32;
            let mapped = (value + 1.0).ln() * scale;
            dst.put_pixel(x, y, Luma([mapped.clamp(0.0, 255.0) as u8]));
        }
    }
    dst
}

/// Gamma 变换：s = 255 * (r / 255)^gamma。
pub fn gen_gamma_trans(src: &RgbImage, gamma: f32) -> RgbImage {
    let mut dst = RgbImage::new(src.width(), src.height());
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
    dst
}

/// 手动计算 Otsu 阈值。
pub fn otsu_threshold(gray: &GrayImage) -> u8 {
    let hist = calculate_histogram(gray);
    let total: u32 = hist.iter().copied().sum();
    let sum: f32 = hist
        .iter()
        .enumerate()
        .map(|(i, &count)| i as f32 * count as f32)
        .sum();

    let mut sum_b = 0.0;
    let mut w_b = 0u32;
    let mut w_f;
    let mut max_var = 0.0;
    let mut threshold = 0u8;

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

    threshold
}

/// 二值化转换。type_flag：0=正向二值，1=反向二值，2=Otsu 自动阈值。
pub fn gen_threshold(src: &RgbImage, thresh: u8, type_flag: u8) -> GrayImage {
    let gray = to_grayscale(src);
    let threshold = if type_flag == 2 {
        otsu_threshold(&gray)
    } else {
        thresh
    };
    let mut dst = GrayImage::new(gray.width(), gray.height());
    for y in 0..gray.height() {
        for x in 0..gray.width() {
            let value = gray.get_pixel(x, y)[0];
            let out = match type_flag {
                1 => {
                    if value > threshold {
                        0
                    } else {
                        255
                    }
                }
                _ => {
                    if value > threshold {
                        255
                    } else {
                        0
                    }
                }
            };
            dst.put_pixel(x, y, Luma([out]));
        }
    }
    dst
}

/// 分段线性灰度映射。使用 LUT 将每个像素值转换。
pub fn gen_piecewise_lin(src: &RgbImage, r1: u8, s1: u8, r2: u8, s2: u8) -> RgbImage {
    let mut lut = [0u8; 256];
    for i in 0..256 {
        lut[i] = if i < r1 as usize {
            ((s1 as f32 / r1.max(1) as f32) * i as f32).round() as u8
        } else if i < r2 as usize {
            (((s2 as f32 - s1 as f32) / (r2 as f32 - r1 as f32)) * (i as f32 - r1 as f32)
                + s1 as f32)
                .round() as u8
        } else {
            (((255.0 - s2 as f32) / (255.0 - r2 as f32)) * (i as f32 - r2 as f32) + s2 as f32)
                .round() as u8
        }
    }

    let mut dst = RgbImage::new(src.width(), src.height());
    for y in 0..src.height() {
        for x in 0..src.width() {
            let [r, g, b] = src.get_pixel(x, y).0;
            dst.put_pixel(
                x,
                y,
                Rgb([lut[r as usize], lut[g as usize], lut[b as usize]]),
            );
        }
    }
    dst
}

/// 直方图均衡化。
pub fn gen_equalize_hist(src: &RgbImage) -> GrayImage {
    let gray = to_grayscale(src);
    let hist = calculate_histogram(&gray);
    let total = gray.width() * gray.height();
    let mut cdf = [0u32; 256];
    let mut cumulative = 0u32;
    for i in 0..256 {
        cumulative += hist[i];
        cdf[i] = cumulative;
    }
    let cdf_min = cdf.iter().copied().find(|&v| v > 0).unwrap_or(0) as f32;
    let total_f = total as f32;

    let mut lut = [0u8; 256];
    for i in 0..256 {
        let numerator = cdf[i] as f32 - cdf_min;
        let mapped = ((numerator / (total_f - cdf_min)) * 255.0).round();
        lut[i] = mapped.clamp(0.0, 255.0) as u8;
    }

    let mut dst = GrayImage::new(gray.width(), gray.height());
    for y in 0..gray.height() {
        for x in 0..gray.width() {
            let value = gray.get_pixel(x, y)[0];
            dst.put_pixel(x, y, Luma([lut[value as usize]]));
        }
    }
    dst
}

/// 运行 Chapter 3 的图像处理流程，并保存输出文件。
pub fn run_chapt3(img: &RgbImage, output_root: &Path) -> Result<(), ImageError> {
    let gray = to_grayscale(img);
    let hist = calculate_histogram(&gray);
    let hist_img = draw_histogram(&hist);
    hist_img.save(output_root.join("chapt3_histogram.png"))?;

    let linear = gen_gray_lin_trans(img, 1.2, 20.0);
    let log = gen_gray_log_trans(img);
    let gamma = gen_gamma_trans(img, 2.5);
    let threshold = gen_threshold(img, 1, 1);
    let piecewise = gen_piecewise_lin(img, 50, 20, 200, 240);
    let equalized = gen_equalize_hist(img);

    linear.save(output_root.join("chapt3_linear.png"))?;
    log.save(output_root.join("chapt3_log.png"))?;
    gamma.save(output_root.join("chapt3_gamma.png"))?;
    threshold.save(output_root.join("chapt3_threshold.png"))?;
    piecewise.save(output_root.join("chapt3_piecewise.png"))?;
    equalized.save(output_root.join("chapt3_equalized.png"))?;

    Ok(())
}
