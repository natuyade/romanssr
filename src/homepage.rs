use leptos::prelude::*;
#[component]
pub fn homepage() -> impl IntoView{
    let (kon, set_kon) = signal(true);
    view! {
        /* 
         * signalを扱うときは必ずmoveクロージャ.
         * 条件分岐などに使う!リアクティブな物には必ずget_untracked()
         * 
         * leptowを実際にwebで動かす場合書き方が厳しいのできちんと使い分けする
         */ 
        <h1>"CONにちwow!"</h1>
        <button
            on:click=move |_| set_kon.set(!kon.get_untracked())
        >{ move || kon.get() }
        </button>
        <Show when=move || kon.get()>
            <p>"Show when=move || kon.get()"</p>
        </Show>
        <Show when=move || !kon.get()>
            <p>"Show when=move || !kon.get()"</p>
        </Show>
    }
}