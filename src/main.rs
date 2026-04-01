mod chapts;
mod common;

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::common::to_grayscale;

/// 将工作区根目录作为基准，避免运行时路径错误。
fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap_or(&manifest_dir).to_path_buf()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = workspace_root();
    let image_path = root.join("cvcls_rs/static/image.png");
    let output_root = root.join("cvcls_rs/output");
    fs::create_dir_all(&output_root)?;

    let dyn_image = image::open(&image_path).map_err(|err| {
        eprintln!(
            "[Error] Cannot read the image from {}: {}",
            image_path.display(),
            err
        );
        err
    })?;
    let rgb_image = dyn_image.to_rgb8();
    let gray_image = to_grayscale(&rgb_image);

    if !common::check_valid(&rgb_image) {
        eprintln!("[Error] Invalid image.");
        return Ok(());
    }

    chapts::chapt5::run_chapt5(&gray_image, &output_root)?;
    println!("[CHPT] Chapt 5 Generated!");

    chapts::chapt2::run_chapt2(&rgb_image, &output_root)?;
    println!("[CHPT] Chapt 2 Generated!");

    chapts::chapt3::run_chapt3(&rgb_image, &output_root)?;
    println!("[CHPT] Chapt 3 Generated!");

    chapts::chapt4::run_chapt4(&rgb_image, &output_root)?;
    println!("[CHPT] Chapt 4 Generated!");

    let gray = common::to_grayscale(&rgb_image);

    let mut report = fs::File::create(output_root.join("report.txt"))?;
    writeln!(
        report,
        "Rust CV project output saved to {}",
        output_root.display()
    )?;
    writeln!(
        report,
        "Basic info: width={}, height={}, channels=3",
        rgb_image.width(),
        rgb_image.height()
    )?;
    writeln!(
        report,
        "Pixel at (297,260): {:?}",
        common::get_pixel_value(&rgb_image, 297, 260)
    )?;
    writeln!(report, "Is grayscale: {}", common::is_grayscale(&rgb_image))?;
    writeln!(report, "Is binary: {}", common::is_binary(&gray))?;
    writeln!(
        report,
        "Otsu threshold: {}",
        chapts::chapt3::otsu_threshold(&gray)
    )?;

    println!("Done. Files saved to {}.", output_root.display());
    Ok(())
}
