use crate::common::utils::files::{get_upload_folder_path, PathMode};
use crate::types::AppError;
use std::path::Path;
use std::time::Duration;
use tokio::fs::remove_dir_all;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn upload_job() -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;
    // TODO: clear the folder once a day (00:00)
    sched
        .add(Job::new_repeated_async(
            Duration::from_secs(3600),
            |_, _| {
                Box::pin(async move {
                    clean_upload_dir()
                        .await
                        .expect("Error: clean up upload dir");
                })
            },
        )?)
        .await?;

    sched.start().await?;

    Ok(())
}

async fn clean_upload_dir() -> Result<(), AppError> {
    let mut read_dir = tokio::fs::read_dir(Path::new("public/uploads")).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let dir_path = entry.path().display().to_string();
        let current_dir = get_upload_folder_path(PathMode::Upload);
        let is_dir = entry.metadata().await.unwrap().is_dir();

        if dir_path != current_dir && is_dir {
            remove_dir_all(entry.path())
                .await
                .expect("Error: remove dir");
        }
    }

    Ok(())
}
