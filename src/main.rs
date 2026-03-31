mod chapt2;
mod chapt3;
mod chapt4;
mod common;

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

    if !common::check_valid(&rgb_image) {
        eprintln!("[Error] Invalid image.");
        return Ok(());
    }

    chapt2::run_chapt2(&rgb_image, &output_root)?;
    chapt3::run_chapt3(&rgb_image, &output_root)?;
    chapt4::run_chapt4(&rgb_image, &output_root)?;

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
    writeln!(report, "Otsu threshold: {}", chapt3::otsu_threshold(&gray))?;

    println!("处理完成，结果保存到 {}", output_root.display());
    Ok(())
}
