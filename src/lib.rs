// appはlib,mainでの共通componentなのでpub
pub mod app;
mod homepage;

// wasm/hydrate entry用
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(app::App);
}
