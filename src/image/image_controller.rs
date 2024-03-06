use crate::image::image_service::convert_image;
use ntex::web;

pub fn image_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/image").service(convert_image));
}
