use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use validator::Validate;
extern crate image;

#[derive(Debug, Deserialize, Serialize)]
enum AsType {
    Base64,
    FilePath,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
struct MyData {
    convert_to: String,
    image_file: Vec<u8>,
    as_type: AsType,
}

#[derive(Debug, Serialize)]
struct ImageResponse {
    image_base_64: String,
}
#[handler]
pub async fn convert_image(req: &mut Request, res: &mut Response) {
    req.form::<String>("id").await;
    let (convert_to, image_file) = (
        req.form::<String>("convert_to")
            .await
            .unwrap_or("png".to_string()),
        req.file("image_file").await,
    );

    if let Some(file) = image_file {
        let input_image: DynamicImage = ImageReader::open(file.path()).unwrap().decode().unwrap();

        let mut output_buffer = Vec::new();
        input_image
            .write_to(&mut Cursor::new(&mut output_buffer), ImageFormat::Bmp)
            .expect("Failed to convert image format");

        let res_base64 = STANDARD.encode(&output_buffer);
        let image_base_64 = format!(
            "data:image/{};base64,{}",
            convert_to.to_lowercase(),
            res_base64
        );

        res.render(Json(ImageResponse { image_base_64 }))
    }
}
