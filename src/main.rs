use salvo::cors::Cors;
use salvo::http::Method;
use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
//  IMAGE
mod image;
use crate::image::image_controller::image_controller;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    let router = Router::new()
        .push(
            Router::with_path("<**path>").get(
                StaticDir::new(["public"])
                    .defaults("index.html")
                    .auto_list(true),
            ),
        )
        .push(Router::with_path("api/v1").push(image_controller()));

    let service = Service::new(router).hoop(Logger::new()).hoop(cors);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(service).await;
}
