use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use validator::{Validate, ValidationError};

extern crate image;

#[derive(Debug, Validate, Deserialize, Serialize)]
struct MyData {
    #[validate(required, custom = "validate_role")]
    role: Option<String>,
}

fn validate_role(role: &str) -> Result<(), ValidationError> {
    if role == "admin" || role == "user" {
        Ok(())
    } else {
        Err(ValidationError::new("terrible_username"))
    }
}
#[handler]
pub async fn convert_image(req: &mut Request, res: &mut Response) {
    let form = req.parse_form::<MyData>().await.unwrap().validate();

    match form {
        Ok(_) => {
            let file = req.file("file").await;
            if let Some(file) = file {
                let input_image: DynamicImage =
                    ImageReader::open(file.path()).unwrap().decode().unwrap();

                let mut output_buffer = Vec::new();
                input_image
                    .write_to(&mut Cursor::new(&mut output_buffer), ImageFormat::Bmp)
                    .expect("Failed to convert image format");

                res.write_body(output_buffer).unwrap()
            }
        }
        Err(e) => res.status_code(StatusCode::BAD_REQUEST).render(Json(e)),
    };
}
