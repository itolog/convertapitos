use crate::image::types::ImageResponse;
use crate::image::utils::{get_save_file_path, get_upload_folder_path, match_ext, PathMode};
use crate::types::{AppResponse, AppResult, DataErr};

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use salvo::prelude::*;
use std::path::Path;
use tokio::fs::create_dir_all;

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
        let upload_folder = get_upload_folder_path(PathMode::Upload);

        create_dir_all(Path::new(&upload_folder)).await?;

        let (file_name, _ext) = file.name().unwrap().split_once('.').unwrap();
        let (save_file_path, image_link) = get_save_file_path(&convert_to.to_lowercase());

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
