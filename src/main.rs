use salvo::prelude::*;

#[handler]
async fn convert_image() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().path("convert_image").get(convert_image);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}