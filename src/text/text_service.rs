use crate::common::utils::files::{get_save_file_path, get_upload_folder_path, PathMode};
use crate::text::types::FileResponse;

use crate::common::types::api_types::{AppResponse, DataErr};
use crate::common::types::app_types::AppResult;
use salvo::prelude::*;
use std::path::Path;
use tokio::fs::create_dir_all;
use tts_rust::languages::Languages;
use tts_rust::tts::GTTSClient;

#[handler]
pub async fn text_to_speech(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let text = req.form::<String>("text").await.unwrap();
    let file_ext = String::from("mp3");

    let upload_folder = get_upload_folder_path(PathMode::Upload);
    let (save_file_path, file_link, file_name) = get_save_file_path(&file_ext);

    create_dir_all(Path::new(&upload_folder)).await?;

    let narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
        tld: "com",
    };

    let file_res = narrator.save_to_file(&text, &save_file_path);

    match file_res {
        Ok(_file) => {
            res.render(Json(AppResponse::<FileResponse> {
                data: FileResponse {
                    file_link,
                    file_name,
                },
                error: None,
            }));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(AppResponse::<Option<FileResponse>> {
                data: None,
                error: Option::from(DataErr {
                    message: e.to_string(),
                    status: 400,
                }),
            }));
        }
    }

    Ok(())
}
