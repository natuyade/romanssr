use wasm_bindgen::prelude::wasm_bindgen;

mod app;
use app::App;

mod globalcss;
mod p2r_menu;
mod pre_date;
mod homepage;
mod nonsense;

/// クライアントから呼ばれるエントリポイント
#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    web_sys::console::log_1(&"hydration started".into());
    leptos::logging::log!("HYDRATE START");
    leptos::mount::mount_to_body(App)
}
