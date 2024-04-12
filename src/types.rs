use image::ImageError;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::string::FromUtf8Error;
use thiserror::Error;
use tokio::io;
use tokio_cron_scheduler::JobSchedulerError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("io: `{0}`")]
    Io(#[from] io::Error),
    #[error("utf8: `{0}`")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("ImageError: `{0}`")]
    ImageError(#[from] ImageError),
    #[error("JobSchedulerError: `{0}`")]
    JobSchedulerError(#[from] JobSchedulerError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataErr {
    pub message: String,
    pub status: i16,
}

#[derive(Debug, Serialize)]
pub struct AppResponse<T, E> {
    pub data: T,
    pub error: E,
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let response = AppResponse::<Option<String>, DataErr> {
            data: None,
            error: DataErr {
                message: self.to_string(), //TODO: do not show system errors to the user
                status: 500,
            },
        };

        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(response));
    }
}

pub type AppResult<T> = Result<T, AppError>;
