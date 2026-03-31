use crate::common::{
    create_blank_image, get_info, get_pixel_value, is_binary, is_grayscale, resize_image,
    set_pixel_value, to_grayscale,
};
use image::{ImageError, RgbImage};
use std::path::Path;

/// 运行 Chapter 2 的图像处理流程，并保存输出文件。
pub fn run_chapt2(img: &RgbImage, output_root: &Path) -> Result<(), ImageError> {
    get_info(img, 3);
    if let Some(pixel) = get_pixel_value(img, 297, 260) {
        println!("Pixel value at (297,260): {:?}", pixel);
    }

    let mut modified = img.clone();
    for y in 0..100u32.min(modified.height()) {
        for x in 0..100u32.min(modified.width()) {
            let _ = set_pixel_value(&mut modified, y, x, [0, 255, 255]);
        }
    }
    modified.save(output_root.join("chapt2_set_pixel.png"))?;
    println!("[TASK] Set pixel task completed!");

    let resized = resize_image(img, 320, 240);
    resized.save(output_root.join("chapt2_resized.png"))?;
    println!("[TASK] Resize task completed!");

    let blank = create_blank_image(320, 240, [0, 0, 255]);
    blank.save(output_root.join("chapt2_blank.png"))?;
    println!("[TASK] Create blank task completed!");

    println!("Is grayscale: {}", is_grayscale(img));
    let gray = to_grayscale(img);
    gray.save(output_root.join("chapt2_gray.png"))?;
    println!("[TASK] Change to gray task completed!");

    println!("Is binary: {}", is_binary(&gray));

    Ok(())
}
