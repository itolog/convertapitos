use crate::image::utils::{
    get_save_file_path, get_upload_folder_path, match_ext, validate_file, PathMode,
};

use crate::common::types::api_types::{AppResponse, DataErr, ImageResponse};
use crate::common::types::app_types::AppResult;
use image::imageops::FilterType;
use image::DynamicImage;
use image::ImageReader;
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
        let is_file_valid = validate_image_file(file);

        if is_file_valid {
            let upload_folder = get_upload_folder_path(PathMode::Upload);

            create_dir_all(Path::new(&upload_folder)).await?;

            let (save_file_path, image_link, file_name) =
                get_save_file_path(&convert_to.to_lowercase());

            let mut input_image: DynamicImage =
                ImageReader::open(Path::new(file.path()))?.decode()?;

            if convert_to == "ico" {
                input_image = input_image.resize(256, 256, FilterType::Nearest);
            }

            input_image.save_with_format(Path::new(&save_file_path), match_ext(&convert_to))?;

            res.render(Json(AppResponse::<ImageResponse> {
                data: ImageResponse {
                    file_name,
                    image_link,
                },
                error: None,
            }));
        } else {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(AppResponse::<Option<ImageResponse>> {
                data: None,
                error: Option::from(DataErr {
                    message: "Incorrect file".to_string(),
                    status: 400,
                }),
            }));
        }
    }

    Ok(())
}
