use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn p2r_menu() -> impl IntoView {
    // CSR 用 signal のみ wasm32 で作成
    #[cfg(target_arch = "wasm32")]
    let (menu, set_menu) = signal(false);
    #[cfg(target_arch = "wasm32")]
    let (menu_anim, set_menu_anim) = signal(false);

    // view! は常に一つ
    view! {
        <div class="menuwrap">
            <img
                // src は式で SSR/CSR を分ける
                src=move || {
                    // CSR
                    #[cfg(target_arch = "wasm32")]
                    {
                        if menu.get() {
                            "assets/images/menu_on.webp"
                        } else {
                            "assets/images/menu_off.webp"
                        }
                    }
                    // SSR
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        "assets/images/menu_off.webp"
                    }
                }
                class="menu-icon"
                // CSR 用 reactive class
                on:click=move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        set_menu.set(!menu.get());
                        set_menu_anim.set(true);
                    }
                }
                on:animationend=move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        set_menu_anim.set(false);
                    }
                }
                // CSR でのみ class:menu-anim を反映
                class:menu-anim=move || {
                    #[cfg(target_arch = "wasm32")]
                    {
                        menu_anim.get()
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        false
                    }
                }
            />
        </div>
    }
}