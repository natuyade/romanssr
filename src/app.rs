use crate::homepage::Homepage;

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

// ssrとcsr時でhtml展開後のコードが一致しない場合正しく読み込まれない
#[component]
pub fn App() -> impl IntoView {

    let (count, set_count) = signal(0);
    view! { 
                        <h1>"Hello from Leptos"</h1>
                            <button on:click=move |_| {
                                set_count.update(|c| *c += 1);
                            }>
                                { move || count.get() }
                            </button>
                                <Router>
                                    <Routes fallback = || "Page not found">
                                        <Route path=path!("/") view=Homepage/>
                                        <Route path=path!("/homepage") view=Homepage/>
                                    </Routes>
                                </Router>
    }
}
