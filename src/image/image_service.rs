use crate::image::types::ImageResponse;
use crate::image::utils::match_ext;
use crate::types::{AppResponse, AppResult, DataErr};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use salvo::prelude::*;
use std::path::Path;

extern crate image;

#[handler]
pub async fn convert_image(req: &mut Request, res: &mut Response) -> AppResult<()> {
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

        let mut input_image: DynamicImage = ImageReader::open(Path::new(file.path()))?.decode()?;

        if convert_to == "ico" {
            input_image = input_image.resize(256, 256, FilterType::Nearest);
        }

        input_image.save_with_format(Path::new(&save_file_path), match_ext(&convert_to))?;

        res.render(Json(AppResponse::<ImageResponse, Option<DataErr>> {
            data: Some(ImageResponse {
                file_name: format!("{}.{}", file_name, convert_to),
                image_link,
            }),
            error: None,
        }));
    }

    Ok(())
}
