use wasm_bindgen::prelude::*;
use surt_rs::generate_surt;


/// This function takes a URL and returns its SURT form.
///
/// ### Basic Example
///
/// ```js
/// import { surt } from 'surt-rs';
/// 
/// console.log(surt('http://example.com/'));
/// // http://example.com/
/// ```
///
#[wasm_bindgen]
pub fn surt(url: &str) -> String {
    generate_surt(url).unwrap_or_else(|_| url.to_string())
}
