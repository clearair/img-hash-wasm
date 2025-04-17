use img_hash::{HashAlg, HasherConfig};
use wasm_bindgen::prelude::*;
use img_hash::image::{ImageBuffer, Rgba};

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub struct ImgByte {
    bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
impl ImgByte {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Box<[u8]>, width: u32, height: u32) -> ImgByte {
        ImgByte { bytes: bytes.to_vec(), width, height }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    #[wasm_bindgen(setter)]
    pub fn set_bytes(&mut self, bs: &[u8]) {
        self.bytes = bs.to_vec();
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}
#[wasm_bindgen]
pub fn compare_images(img1_bytes: &ImgByte, img2_bytes: &ImgByte) -> f64 {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // web_sys::console::log_1(&format!("img1 len: {}", img1_bytes.bytes.len()).into());
    // web_sys::console::log_1(&format!("img2 len: {}", img2_bytes.bytes.len()).into());

    let img1 = ImageBuffer::<Rgba<u8>, _>::from_raw(img1_bytes.get_width(), img1_bytes.get_height(), img1_bytes.get_bytes()).unwrap();
    let img2 = ImageBuffer::<Rgba<u8>, _>::from_raw(img2_bytes.get_width(), img2_bytes.get_height(), img2_bytes.get_bytes()).unwrap();

    let dyn_img1 = img_hash::image::DynamicImage::ImageRgba8(img1);
    let dyn_img2 = img_hash::image::DynamicImage::ImageRgba8(img2);

    let hasher = HasherConfig::new().hash_alg(HashAlg::Gradient).to_hasher();
    let hash1 = hasher.hash_image(&dyn_img1);
    let hash2 = hasher.hash_image(&dyn_img2);

    let bit_len = hash1.as_bytes().len() * 8;

    hash1.dist(&hash2) as f64 / bit_len as f64
    
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[cfg(all(test, target_arch = "wasm32"))]
use wasm_bindgen_test::wasm_bindgen_test_configure;

#[cfg(all(test, target_arch = "wasm32"))]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use super::*;
    use img_hash::image::{RgbaImage, Rgba};

    fn create_test_image(color: [u8; 4], width: u32, height: u32) -> ImgByte {
        let mut img = RgbaImage::new(width, height);
        for x in 0..width {
            for y in 0..height {
                img.put_pixel(x, y, Rgba(color));
            }
        }
        let bytes = img.into_raw().into_boxed_slice();
        ImgByte::new(bytes, width, height)
    }

    #[test]
    fn test_compare_identical_images() {
        let img1 = create_test_image([255, 0, 0, 255], 8, 8);
        let img2 = create_test_image([255, 0, 0, 255], 8, 8);
        let score = compare_images(&img1, &img2);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_compare_different_images() {
        let img1 = create_test_image([255, 0, 0, 255], 8, 8); // 红色
        let img2 = create_test_image([0, 255, 0, 255], 8, 8); // 绿色
        let score = compare_images(&img1, &img2);
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
}
