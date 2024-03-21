pub mod app;

#[cfg(feature = "ssr")]
pub mod ssr;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init().ok();

    mount_to_body(App);
}
