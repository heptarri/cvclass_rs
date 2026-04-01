# Copilot Instructions for cvcls_rs

## Project Overview

This is a computer vision course project for Chang'an University's Robotics Engineering program, implementing image processing algorithms from scratch in Rust. The project demonstrates various CV techniques organized by course chapters.

## Build and Run

```bash
# Build the project
cargo build

# Run the project (processes static/image.png and outputs to ./output/)
cargo run

# Check for errors without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

The project does not have tests - it's a coursework project that produces image outputs.

## Architecture

### Module Structure

```
src/
├── main.rs           # Entry point: loads image, runs all chapters, generates report
├── common.rs         # Shared image utilities and operations
└── chapts/           # Course chapter implementations
    ├── mod.rs
    ├── chapt2.rs     # Basic image operations
    ├── chapt3.rs     # Histogram & intensity transformations
    ├── chapt4.rs     # Geometric transformations
    └── chapt5.rs     # Spatial filters & edge detection
```

### Execution Flow

1. `main.rs` loads `static/image.png` into memory
2. Converts to RGB format using the `image` crate
3. Generates grayscale version via `common::to_grayscale()`
4. Sequentially runs all chapter modules (chapt5, then chapt2-4)
5. Each chapter saves output images to `./output/`
6. Creates `output/report.txt` with image statistics

**Key detail**: All algorithms are manually implemented - do not use `image` crate's built-in filters or transformations.

### Chapter Responsibilities

- **chapt2**: Pixel manipulation, resizing, blank image creation, grayscale conversion
- **chapt3**: Histogram calculation/visualization, intensity transformations (linear, log, gamma, piecewise), Otsu thresholding, histogram equalization
- **chapt4**: Geometric transforms (translation, flip, transpose, rotation), interpolation methods (nearest, bilinear, bicubic)
- **chapt5**: Spatial filtering (mean, Gaussian, median), edge detection (Robert, Sobel, Laplacian)

## Key Conventions

### Image Coordinate System

- Functions use `(row, col)` or `(y, x)` order for coordinates
- The `image` crate uses `(x, y)` internally, so conversions flip the order:
  ```rust
  // Getting a pixel at row=297, col=260
  get_pixel_value(img, 297, 260) // Returns Some([u8; 3])
  // Internally calls: img.get_pixel(col, row)
  ```

### Manual Algorithm Implementation

All image processing algorithms are hand-coded:
- No use of `imageproc` or similar high-level CV libraries
- Direct pixel manipulation via nested loops
- Manual kernel convolution in `chapt5::img_filter()`
- Custom interpolation logic (nearest, bilinear, bicubic)

When adding new algorithms, follow this pattern:
1. Iterate through pixels with nested `for` loops
2. Apply transformation/filter logic
3. Clamp output values to `[0, 255]` range
4. Return new image (avoid mutation when possible)

### Module Organization

Each chapter module exports:
- Individual algorithm functions (e.g., `img_mean()`, `gen_gamma_trans()`)
- A single `run_chaptN()` function that executes the chapter's demos and saves outputs
- All functions work with `&RgbImage` or `&GrayImage` from the `image` crate

### Error Handling

- Most functions return `Result<(), ImageError>` at the top level
- Image I/O operations propagate errors with `?`
- Pixel operations return `Option` or `bool` for out-of-bounds checks
- Console logging uses `println!` with prefixes like `[TASK]`, `[CHPT]`, `[Error]`

### Path Handling

The project expects to be inside a workspace directory:
```rust
fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap_or(&manifest_dir).to_path_buf()
}
```

All file paths are constructed relative to `workspace_root()`:
- Input: `{workspace_root}/cvcls_rs/static/image.png`
- Output: `{workspace_root}/cvcls_rs/output/`

### Chinese Comments

Source code contains Chinese comments describing algorithms and formulas. This is intentional for educational purposes - maintain bilingual comments when adding new code.

## Common Utilities (`common.rs`)

Reusable functions for all chapters:
- `to_grayscale()`: Weighted RGB to grayscale conversion (0.299R + 0.587G + 0.114B)
- `check_valid()`: Validates image dimensions are > 0
- `get_pixel_value()` / `set_pixel_value()`: Safe pixel access with bounds checking
- `resize_image()`: Nearest-neighbor scaling
- `is_grayscale()` / `is_binary()`: Image type validation

When adding utilities, ensure they're generic over `GenericImageView` when possible for reusability.
