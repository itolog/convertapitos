use crate::types::types_service::get_types;
use salvo::Router;

pub fn types_controller() -> Router {
    Router::with_path("types").get(get_types)
}
