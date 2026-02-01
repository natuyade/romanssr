use leptos::prelude::*;

use crate::globalcss::global_style;
use crate::homepage::HomePage;
use crate::p2r_menu::p2r_menu;

/* 
 * signalを.get()した際に依存関係が構築されるためその値が変わる度に更新されてしまう
 *  (本来は読み取るだけでなので更新される必要はない->警告が出る(or panic))
 * そこで.get_untracked()使いリアクティブ処理で依存関係がないので安全に値を読み取れる
 */
#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>
            { global_style() }
        </style>
        <main>
            { p2r_menu() }
            { HomePage }
        </main>
    }
}