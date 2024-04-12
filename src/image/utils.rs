use chrono::{Datelike, Local, Timelike};
use image::ImageFormat;
use rand::distributions::{Alphanumeric, DistString};
use salvo::http::form::FilePart;

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

pub fn get_save_file_path(ext: &String) -> (String, String, String) {
    let rand_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);

    let now = Local::now();
    let file_name = format!(
        "{:02}-{:02}-{:02}_{}_image.{}",
        now.hour(),
        now.minute(),
        now.second(),
        rand_str,
        ext
    );

    let save_file_path = format!("{}/{}", get_upload_folder_path(PathMode::Upload), file_name);

    let image_link = format!(
        "{}/{}",
        get_upload_folder_path(PathMode::Response),
        file_name
    );

    (save_file_path, image_link, file_name)
}

pub fn validate_file(file: &FilePart) -> bool {
    let valid_types = [
        String::from("image/avif"),
        String::from("image/bmp"),
        String::from("image/dds"),
        String::from("image/gif"),
        String::from("image/hdr"),
        String::from("image/ico"),
        String::from("image/jpeg"),
        String::from("image/exr"),
        String::from("image/png"),
        String::from("image/pnm"),
        String::from("image/qoi"),
        String::from("image/tga"),
        String::from("image/tiff"),
        String::from("image/webp"),
    ];
    let file_content_type = file.content_type().unwrap().to_string();

    if valid_types.contains(&file_content_type) {
        return true;
    }

    false
}
