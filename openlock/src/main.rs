use anyhow::Result;
use std::sync::Arc;

use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use ethers::prelude::*;
use leptos::*;

#[derive(Clone)]
struct AppState {
    name: String,
    provider: Arc<Provider<Ws>>,
}

#[component]
fn Head(cx: Scope, title: String) -> impl IntoView {
    view! {cx,
        <head>
        </head>
    }
}

#[component]
fn Nav(cx: Scope) -> impl IntoView {
    view! {cx,
        <header class="header">
            <nav>
            </nav>
        </header>
    }
}

#[component]
fn App(cx: Scope, #[prop(into)] block: Signal<u64>) -> impl IntoView {
    view! {cx,
        <body style="color: white; background-color:black">
            <p>{block.get()}</p>
            <button>Connect</button>
        </body>
    }
}

#[get("/style.css")]
async fn css() -> impl Responder {
    actix_files::NamedFile::open_async("static/style.css").await
}
#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    actix_files::NamedFile::open_async("static/favicon.ico").await
}

#[get("/")]
async fn index(_req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let block_number = data.provider.get_block_number().await.unwrap();

    let html = leptos::ssr::render_to_string(move |cx| {
        let (block, _) = create_signal(cx, block_number.as_u64());
        view! {cx,
            <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <title>{&data.name}</title>
                <link href="/favicon.ico" rel="icon" type="image/x-icon"></link>
                <script src="https://unpkg.com/htmx.org@1.9.5" integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" crossorigin="anonymous"></script>
            </head>
                <App block=block/>
            </html>
        }
    });

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        provider: Arc::new(Provider::<Ws>::connect("ws://127.0.0.1:8545").await?),
        name: String::from("OpenLock"),
    });
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .service(favicon)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    return Ok(());
}
