use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileResponse {
    pub file_link: String,
    pub file_name: String,
}
