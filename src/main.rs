use salvo::logging::Logger;
use salvo::prelude::*;
//  IMAGE
mod image;
use crate::image::image_controller;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::with_path("api/v1").push(image_controller());

    let service = Service::new(router).hoop(Logger::new());

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(service).await;
}
