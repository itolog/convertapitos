use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Deserialize, Serialize)]
pub struct DataErr {
    pub message: String,
    pub status: i16,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    pub data: T,
    pub error: Option<DataErr>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResponse {
    pub image_link: String,
    pub file_name: String,
}
