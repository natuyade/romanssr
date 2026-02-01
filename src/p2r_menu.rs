use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn p2r_menu() -> impl IntoView {
    let (menu, set_menu) = signal(false);
    let (menu_anim, set_menu_anim) = signal(false);

    view! {
        <div class="menuwrap">
            <img
                // src は式で SSR/CSR を分ける
                src=move || {
                        if menu.get() {
                            "assets/images/menu_on.webp"
                        } else {
                            "assets/images/menu_off.webp"
                        }
                }
                class="menu-icon"
                on:click=move |_| {
                    {
                        set_menu.set(!menu.get());
                        set_menu_anim.set(true);
                    }
                }
                on:animationend=move |_| {
                    {
                        set_menu_anim.set(false);
                    }
                }
                class:menu-anim=move || {
                        menu_anim.get()
                    }
            />
        </div>
    }
}