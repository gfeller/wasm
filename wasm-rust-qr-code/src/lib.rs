mod utils;

use wasm_bindgen::prelude::*;
 
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct SampleQrCode {
    width: u16,
    height: u16,
    buffer: Vec<u8>
}



#[wasm_bindgen]
impl SampleQrCode {
    pub fn new(value: &str) -> SampleQrCode {

        utils::set_panic_hook();


        let qr_code = qr_code::QrCode::new(value).unwrap();

        let height: u16 = qr_code.width() as u16;
        let width: u16 = qr_code.width() as u16;
        let size: usize = (height * width * 4).into();

        let mut buffer: Vec<u8> = vec![0; size];
       
        let mut pos = 0;
        for pixel_set in qr_code.into_colors() {
            let pixel_value = if pixel_set == qr_code::Color::Light { 255 } else { 0 };
            buffer[pos] = pixel_value;   // Red
            buffer[pos+1] = pixel_value; // Green
            buffer[pos+2] = pixel_value; // Blue
            buffer[pos+3] = 255;         // Alpha
            pos += 4;
        }

        SampleQrCode { 
            width,
            height,
            buffer
        }
        
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn buffer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }
}