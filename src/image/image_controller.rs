use crate::image::convert_image;
use salvo::Router;

pub fn image_controller() -> Router {
    Router::new()
        .path("image")
        .push(Router::with_path("convert").post(convert_image))
}
