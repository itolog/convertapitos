use crate::common::utils::jobs::upload_job;
use dotenv::dotenv;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

mod common;
mod image;
// TEXT
mod text;

use crate::image::image_controller::image_controller;
use crate::types::types_controller::types_controller;
use crate::text::text_controller::text_controller;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    dotenv().ok();
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("5800".to_string());
    let local_addr = format!("{}:{}", host, port);

    upload_job().await.expect("Upload job error");

    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    let router = Router::new()
        .push(Router::new().path("api/v1").push(image_controller()))
        .push(Router::new().path("api/v1").push(types_controller()))
        .push(
            Router::new().path("<**path>").get(
                StaticDir::new(["public"])
                    .defaults("index.html")
                    .auto_list(true),
            ),
        );
        )
        .push(
            Router::with_path("api/v1")
                .push(image_controller())
                .push(text_controller()),
        );

    let service = Service::new(router).hoop(Logger::new()).hoop(cors);

    let acceptor = TcpListener::new(local_addr).bind().await;
    Server::new(acceptor).serve(service).await;
}
