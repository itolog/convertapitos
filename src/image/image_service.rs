use futures::TryStreamExt;
// use image::io::Reader as ImageReader;
// use image::{DynamicImage, ImageFormat};
use ntex::web::{self, Error, HttpResponse};
use ntex_multipart::Multipart;
// use std::io::Cursor;

extern crate image;

#[web::post("/convert")]
pub async fn convert_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_type();
        println!("{:?}", content_type.params());
        // let content_type = field.content_disposition().unwrap();
        // let filename = content_type.get_filename().unwrap();
        // let filename = "somename";
        // let filepath = format!("./tmp/{}", filename);
        // File::create is blocking operation, use threadpool
    }
    // let file = req.file("file").await;
    // if let Some(file) = file {
    //     let input_image: DynamicImage = ImageReader::open(file.path()).unwrap().decode().unwrap();
    //
    //     let mut output_buffer = Vec::new();
    //     input_image
    //         .write_to(&mut Cursor::new(&mut output_buffer), ImageFormat::Bmp)
    //         .expect("Failed to convert image format");
    //
    //     res.write_body(output_buffer).unwrap()
    // }

    Ok(HttpResponse::Ok().into())
}
