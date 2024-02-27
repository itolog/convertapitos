use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use salvo::prelude::*;
use std::io::Cursor;

extern crate image;

#[handler]
pub async fn convert_image(req: &mut Request, res: &mut Response) {
    let file = req.file("file").await;
    if let Some(file) = file {
        let input_image: DynamicImage = ImageReader::open(file.path()).unwrap().decode().unwrap();

        let mut output_buffer = Vec::new();
        input_image
            .write_to(&mut Cursor::new(&mut output_buffer), ImageFormat::Bmp)
            .expect("Failed to convert image format");

        res.write_body(output_buffer).unwrap()
    }
}
