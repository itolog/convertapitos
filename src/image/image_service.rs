use crate::image::types::{DataErr, DataResponse, ImageResponse};
use crate::image::utils::match_ext;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use salvo::prelude::*;

extern crate image;

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
