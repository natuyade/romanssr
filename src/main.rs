use axum::{Router, http::HeaderValue, routing::get_service};
use std::net::SocketAddr;
use std::thread::Scope;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

mod app;
mod homepage;
mod nonsense;
mod pre_date;
mod p2r_menu;
mod globalcss;

use app::App;

#[tokio::main]
async fn main() {
    
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let routes = generate_route_list(App);
    
    let cors = CorsLayer::new()
        .allow_origin("https://p2rush.jp".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);
    
    // page.
    let app = Router::new()
        // wasm配信
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        // /assetsにアクセスが来た場合の処理(静的ファイルの提供)
        .nest_service("/assets", ServeDir::new("assets"))
        .leptos_routes(&conf.leptos_options, routes, App)
        .fallback(
            // req ssr handler
            leptos_axum::render_app_to_stream_with_context(
                    || {  },
                    // app route
                    || App,
                )
        )
        .with_state(conf.leptos_options)
        .layer(cors);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
    
}