use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResponse {
    pub image_link: String,
    pub file_name: String,
}
