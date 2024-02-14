
use salvo::prelude::*;
//  IMAGE
mod image;
use crate::image::image_controller::image_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(image_routes());

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}