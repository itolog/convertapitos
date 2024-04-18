use crate::text::text_service::text_to_speech;
use salvo::Router;

pub fn text_controller() -> Router {
    Router::new()
        .path("text")
        .push(Router::with_path("tts").post(text_to_speech))
}
