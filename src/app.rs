use leptos::prelude::*;

// ssrとcsr時でhtml展開後のコードが一致しない場合正しく読み込まれない
#[component]
pub fn App() -> impl IntoView {
    use leptos::prelude::*;

    let (count, set_count) = signal(0);
    view! { 
                        <h1>"Hello from Leptos"</h1>
                        <button on:click=move |_| {
                                    set_count.update(|c| *c += 1);
                                }>
                                    {move || count.get()}
                                </button>
    }
}
