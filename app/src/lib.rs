pub mod app;
pub mod config;
pub mod features;
pub mod shared;

pub use app::App;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::mount::hydrate_body;
    // -- better panic messages in browser console
    console_error_panic_hook::set_once();
    hydrate_body(App);
}
