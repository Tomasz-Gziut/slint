#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() -> Result<(), slint::PlatformError> {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let window = MainWindow::new()?;
    window.run()
}
