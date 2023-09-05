use std::sync::Arc;

use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use ethers::{
    prelude::{LocalWallet, Middleware, Provider, Ws},
    utils::Anvil,
};
use leptos::*;

mod components;
use components::*;

#[derive(Clone)]
struct AppState {
    name: String,
    provider: Arc<Provider<Ws>>,
}

#[component]
fn Head(cx: Scope) -> impl IntoView {
    return view! {cx,
        <head>
            <meta charset="UTF-8" />
            <meta name="description" content="An Open space for Locksport community built" />
            <title>OpenLock</title>
            <link href="/favicon.ico" rel="icon" type="image/x-icon" />
            <link href="/style.css" rel="stylesheet" type="text/css" />
            <script src="https://unpkg.com/htmx.org@1.9.5" integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" crossorigin="anonymous" />
        </head>
    };
}

#[component]
fn Main(cx: Scope) -> impl IntoView {
    return view! {cx,
        <main>
            <UserCard />
            <button>Connect</button>
        </main>
    };
}

#[get("/user")]
async fn user(_req: HttpRequest, _data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // load this from the wallet
    let user = UserItem {
        name: "John".to_string(),
    };
    let html = leptos::ssr::render_to_string(move |cx| {
        view! {cx,
            <UserComponent user=user/>
        }
    });

    return Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}

#[get("/")]
async fn index(_req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let chain_id = data.provider.get_chainid().await.unwrap();
    let block_number = data.provider.get_block_number().await.unwrap();

    let html = leptos::ssr::render_to_string(move |cx| {
        let (chain, _) = create_signal(cx, chain_id.as_u64());
        let (block, _) = create_signal(cx, block_number.as_u64());

        view! {cx,
            <html lang="en">
            <Head />
            <body>
                <Nav />
                <Main />
                <footer>
                    <p>Chain: {chain.get()}</p>
                    <p>Block: {block.get()}</p>
                </footer>
            </body>
            </html>
        }
    });

    return Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}

#[get("/style.css")]
async fn css() -> impl Responder {
    return actix_files::NamedFile::open_async("../static/style.css").await;
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    return actix_files::NamedFile::open_async("../static/favicon.ico").await;
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let provider = Provider::<Ws>::connect("ws://127.0.0.1:8545")
        .await
        .unwrap();

    let app_state = web::Data::new(AppState {
        provider: Arc::new(provider),
        name: String::from("OpenLock"),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .service(favicon)
            .service(index)
            .service(user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    return Ok(());
}
