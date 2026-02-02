// 条件分岐でssr時のみ実行
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes, file_and_error_handler};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tower_http::services::ServeDir;
    
    use natestapp::app::App;
    
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options.clone();
    let routes = generate_route_list(App);
    
    let app = Router::new()
        // leptosroutesで初期のssr配信設定. index等.
        .leptos_routes(&leptos_options, routes, {
                    let _leptos_options = leptos_options.clone();
                    move || leptos::prelude::view! {
                        <!doctype html>
                        <html>
                            <head>
                                <meta charset="utf-8" />
                                <title>じゃれ本部門[P2R]</title>
                                <meta name="viewport" content="width=device-width, initial-scale=1" />
                                <meta name="robots" content="noindex, nofollow" />
                                <link
                                    data-trunk
                                    rel="copy-dir"
                                    href="assets"
                                    data-target-path="assets"
                                />
                                <script type="module">
                                    r#"
                                    import init, { hydrate } from '/pkg/natestapp.js';
                                    init('/pkg/natestapp.wasm').then(() => {
                                        console.log('WASM loaded, calling hydrate...');
                                        hydrate();
                                    });
                                    "#
                                </script>
                            </head>
                                <body>
                                    <App/>
                                </body>
                            </html>
                                
                    }
                })
        // file_and_error_handlerはクロージャで渡さないとエラーを吐く
        .fallback(leptos_axum::file_and_error_handler(|_| {
            leptos::prelude::view! { <App/> }
        }))
        // 静的ファイル配信
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .nest_service("/assets", ServeDir::new("/assets"))
        .with_state(conf.leptos_options);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // CSR/hydrate ビルド時は何もしない
}