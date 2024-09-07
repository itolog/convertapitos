use crate::common::types::app_types::AppError;

use crate::common::utils::files::{get_upload_folder_path, PathMode};
use std::path::Path;
use std::sync::Arc;
use tokio::fs::remove_dir_all;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn upload_job() -> Result<(), JobSchedulerError> {
    let scheduler = JobScheduler::new().await?;
    let arc_scheduler = Arc::new(scheduler);

    // Define a job that runs every 30 minutes
    let job = Job::new_async("0,30 * * * * *", move |_uuid, _l| {
        Box::pin(async move {
            clean_upload_dir()
                .await
                .expect("Error: clean up upload dir");
        })
    })?;

    // Add the job to the scheduler
    arc_scheduler.add(job).await?;

    // Start the scheduler
    let arc_scheduler_clone = Arc::clone(&arc_scheduler);
    tokio::spawn(async move {
        arc_scheduler_clone.start().await.expect("Scheduler Error");
    });

    Ok(())
}

async fn clean_upload_dir() -> Result<(), AppError> {
    let mut read_dir = tokio::fs::read_dir(Path::new("public/uploads")).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let dir_path = entry.path().display().to_string();
        let current_dir = get_upload_folder_path(PathMode::Upload);
        let is_dir = entry.metadata().await?.is_dir();
        println!("dir_path {:?}", dir_path);
        println!("current_dir {:?}", current_dir);
        if dir_path != current_dir && is_dir {
            remove_dir_all(entry.path())
                .await
                .expect("Error: remove dir");
        }
    }

    Ok(())
}
