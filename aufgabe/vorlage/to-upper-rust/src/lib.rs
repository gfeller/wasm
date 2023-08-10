mod utils;

use wasm_bindgen::prelude::*;
 

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// TODO change signatur 
#[wasm_bindgen]
pub extern  fn to_upper(value: i32)  -> i32  {
    return value
}

 