use axum::{Router, http::HeaderValue, routing::get_service};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use leptos_router::components::*;
use leptos_router::path;

use crate::globalcss::global_style;
use crate::homepage::HomePage;
use crate::nonsense::NONS;
use crate::pre_date::PREDATE;
use crate::p2r_menu::p2r_menu;

mod globalcss;
mod homepage;
mod nonsense;
mod pre_date;
mod p2r_menu;

/* 
 * signalを.get()した際に依存関係が構築されるためその値が変わる度に更新されてしまう
 *  (本来は読み取るだけでなので更新される必要はない->警告が出る(or panic))
 * そこで.get_untracked()使いリアクティブ処理で依存関係がないので安全に値を読み取れる
 */
#[component]
fn App() -> impl IntoView {
    view! {
        <style>{ global_style() }</style>
        <main>
            { p2r_menu() }
            { HomePage() }
        </main>
    }
}

fn main() {
    // tokioの非同期runtime作成
    let runtime = tokio::runtime::Builder::new_multi_thread()
        // tokioの機能を全てenableに(I/O,Timer,TaskScheduler,ThreadPool)
        .enable_all()
        // 上記まではただの設定,.build()で構築
        .build()
        .unwrap();
    
    /* 
     * Send = 複数のスレッドに安全に移動できる物
     * future = 未完了の非同期タスク(値)
     * !Sendなfutureはtokioの通常Runtime(multi_thread)で安全に実行できずpanic!します
     * 
     * multi_thread Runtimeは内部でfutureを別のThreadに移動する可能性があり
     * !Sendの値を移動すると危険なためpanicに.
     * 
     * leptosではsignal,Effect等の処理でspawn_local()相当の処理が頻繁に呼ばれる
     * LocalSetは!Sendなfutureを同じスレッドでまとめて管理し実行させるための環境
     * 
     * LocalSetの中ではspawn_local()を使って
     * 同じスレッド上で!Sendなfuture等を実行できます
     */ 
    let local = tokio::task::LocalSet::new();
    // 作成したruntimeをLocalSet環境でasync起動
    local.block_on(&runtime, async {
        
        let conf = get_configuration(Some("Cargo.toml")).unwrap();
        let routes = generate_route_list(App);
        
        let cors = CorsLayer::new()
            .allow_origin("https://p2rush.jp".parse::<HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers(Any);
        
        // page.
        let app = Router::new()
            .leptos_routes(&conf.leptos_options, routes, App)
            .fallback(
                // req ssr handler
                leptos_axum::render_app_to_stream_with_context(
                        || {  },
                        // app route
                        || { App },
                    )
            )
            // /assetsにアクセスが来た場合の処理
            .nest_service("/assets", ServeDir::new("assets"))
            .with_state(conf.leptos_options)
            .layer(cors);
        
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = TcpListener::bind(addr).await.unwrap();
    
        axum::serve(listener, app.into_make_service()).await.unwrap();
    });
}