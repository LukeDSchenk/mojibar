#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod emoji;

pub use app::MojiApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use {
    eframe::wasm_bindgen::{self, prelude::*},
    console_error_panic_hook,
    std::panic,
};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook)); // figure out why this isn't working
    let app = MojiApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
// panicked at 'Could not read emoji.json file to string: Error { kind: Unsupported, message: "operation not supported on this platform" }', src/emoji.rs:19:42
// need to turn emoji-min.json into rust code ^
