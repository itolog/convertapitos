use image::ImageFormat;

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
