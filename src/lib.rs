use wasm_bindgen::prelude::*;
use web_sys::console;
use image::{GrayImage, ImageBuffer, Luma};
use image::{codecs::png::PngEncoder, ImageEncoder}; // ✅ `ImageEncoder` を use
use image::ColorType;
use imageproc::gradients::sobel_gradients;
use imageproc::edges::canny;
use std::io::Cursor;

// Initialization of WebAssembly
#[wasm_bindgen(start)]
pub fn init() {
    console::log_1(&"WASM Module Loaded".into());
}

/// Enum for edge detection method
#[wasm_bindgen]
pub enum EdgeDetectionMethod {
    Sobel,
    Canny,
}

/// Edge detection function for WebAssembly
#[wasm_bindgen]
pub fn process_image_wasm(image_data: &[u8], method: EdgeDetectionMethod) -> Vec<u8> {
    // load image
    // ref. https://docs.rs/image/latest/image/fn.load_from_memory.html
    let img = image::load_from_memory(image_data).expect("Failed to load image");

    // gray scalization
    let gray_img = img.to_luma8();

    // edge detection
    let edge_img_u8 = match method {
        EdgeDetectionMethod::Sobel => {
            let edge_img_u16 = sobel_gradients(&gray_img);
            normalize_u16_to_u8(&edge_img_u16)
        }
        EdgeDetectionMethod::Canny => canny(&gray_img, 50.0, 100.0),
    };

    // encode the image in PNG format and return it as `Vec<u8>`
    let mut buf = Vec::new();
    // Cursor is a type that allows you to write to a buffer in memory
    let mut cursor = Cursor::new(&mut buf);
    // Create a new PNG encoder that writes its output to `cursor`
    let encoder = PngEncoder::new(&mut cursor);
    encoder
        .write_image(&edge_img_u8, edge_img_u8.width(), edge_img_u8.height(), ColorType::L8.into())
        .expect("Failed to encode image");
    buf
}

/// Normalize u16 image to u8
fn normalize_u16_to_u8(img: &ImageBuffer<Luma<u16>, Vec<u16>>) -> GrayImage {
    let (min, max) = img.pixels().fold((u16::MAX, 0), |(min, max), p| {
        let v = p[0];
        (min.min(v), max.max(v))
    });

    if min == max {
        console::log_1(&"Warning: No edge detected (min == max). The output image may be blank.".into());
    }

    GrayImage::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y)[0];
        let scaled_pixel = if max > min {
            ((pixel - min) as f32 / (max - min) as f32 * 255.0) as u8
        } else {
            0
        };
        Luma([scaled_pixel])
    })
}
