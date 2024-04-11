use chrono::{Datelike, Local, Timelike};
use image::ImageFormat;

const UPLOADS_FOLDER_PATH: &str = "public/uploads";
const RESPONSE_FOLDER_PATH: &str = "uploads";

pub fn match_ext(extension: &str) -> ImageFormat {
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

pub enum PathMode {
    Upload,
    Response,
}

pub fn get_upload_folder_path(mode: PathMode) -> String {
    let now = Local::now();

    let current_folder_name = format!(
        "{}-{:02}-{:02}-{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour()
    );

    match mode {
        PathMode::Upload => format!("{}/{}", UPLOADS_FOLDER_PATH, current_folder_name),
        PathMode::Response => format!("{}/{}", RESPONSE_FOLDER_PATH, current_folder_name),
    }
}

pub fn get_save_file_path(ext: &String) -> (String, String) {
    let now = Local::now();
    let file_name = format!(
        "{:02}-{:02}-{:02}_image.{}",
        now.hour(),
        now.minute(),
        now.second(),
        ext
    );

    let save_file_path = format!("{}/{}", get_upload_folder_path(PathMode::Upload), file_name);

    let image_link = format!(
        "{}/{}",
        get_upload_folder_path(PathMode::Response),
        file_name
    );

    (save_file_path, image_link)
}
