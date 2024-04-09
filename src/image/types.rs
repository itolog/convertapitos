use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct MyData<'a> {
    pub convert_to: &'a str,
    pub image_file: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataResponse {
    pub image_link: String,
    pub file_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataErr {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub data: Option<DataResponse>,
    pub error: Option<DataErr>,
}
