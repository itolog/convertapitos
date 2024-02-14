use salvo::{Router};
use crate::image::image_service::hello;

pub fn image_routes() -> Router {
    Router::new()
        .path("image")
        .get(hello)
}
