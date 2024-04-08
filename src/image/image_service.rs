use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
extern crate image;

#[derive(Debug, Deserialize, Serialize)]
enum AsType {
    Base64,
    FilePath,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
struct MyData<'a> {
    convert_to: &'a str,
    image_file: Vec<u8>,
    as_type: AsType,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataResponse {
    image_link: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataErr {
    message: String,
}

#[derive(Debug, Serialize)]
struct ImageResponse {
    data: Option<DataResponse>,
    error: Option<DataErr>,
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
        let (file_name, _ext) = file.name().unwrap().split_once('.').unwrap();
        let save_file_path = format!("public/upload/{}.{}", file_name, convert_to.to_lowercase());
        let image_link = format!("upload/{}.{}", file_name, convert_to.to_lowercase());
        let input_image: DynamicImage = ImageReader::open(file.path()).unwrap().decode().unwrap();

        let _ = input_image
            .save_with_format(save_file_path, match_ext(&convert_to))
            .map(|_| {
                res.render(Json(ImageResponse {
                    data: Some(DataResponse { image_link }),
                    error: None,
                }))
            })
            .map_err(|e| {
                res.render(Json(ImageResponse {
                    data: None,
                    error: Some(DataErr {
                        message: format!("{}", e),
                    }),
                }))
            });
    }
}

fn match_ext(extension: &str) -> ImageFormat {
    match extension {
        "avif" => ImageFormat::Avif,
        "bmp" => ImageFormat::Bmp,
        "dds" => ImageFormat::Dds,
        "gif" => ImageFormat::Gif,
        "hdr" => ImageFormat::Hdr,
        "ico" => ImageFormat::Ico,
        "jpeg" => ImageFormat::Jpeg,
        "exr" => ImageFormat::OpenExr,
        "png" => ImageFormat::Png,
        "pnm" => ImageFormat::Pnm,
        "qoi" => ImageFormat::Qoi,
        "tga" => ImageFormat::Tga,
        "tiff" => ImageFormat::Tiff,
        "webp" => ImageFormat::WebP,
        _ => ImageFormat::Png,
    }
}
