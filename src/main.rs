use env_logger::Env;
use ntex::web;
//  IMAGE
mod image;
use crate::image::image_controller::image_controller;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::Logger::default())
            .service(web::scope("/api/v1").configure(image_controller))
    })
    .bind(("127.0.0.1", 5800))?
    .run()
    .await
}
