use amplify::types::JobStatus;
use amplify::{Client, Error};
use aws_config::meta::region::RegionProviderChain;
use aws_config::{from_env, BehaviorVersion};
use aws_sdk_amplify as amplify;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match establish_connection().await {
        Ok(_) => {
            println!("Successfully connected!");
        }
        Err(e) => {
            println!("Cannot establish connection because {:?}", e);
        }
    }
    Ok(())
}

async fn establish_connection() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = from_env()
        .behavior_version(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);
    let apps = client.list_apps().send().await?;
    println!("Amplify apps: {:?}", apps);
    Ok(())
}

async fn check_amplify_job_status(
    client: &Client,
    app_id: &str,
    branch_name: &str,
    job_id: &str,
) -> Result<String, Error> {
    let job_result = client
        .get_job()
        .app_id(app_id)
        .branch_name(branch_name)
        .job_id(job_id)
        .send()
        .await?;

    if let Some(job) = job_result.job() {
        if let Some(summary) = job.summary() {
            match summary.status() {
                Some(status) => match status {
                    JobStatus::Pending => Ok("Job is pending".to_string()),
                    JobStatus::Provisioning => Ok("Job is provisioning".to_string()),
                    JobStatus::Running => Ok("Job is running".to_string()),
                    JobStatus::Failed => Ok("Job has failed".to_string()),
                    JobStatus::Succeed => Ok("Job has succeeded".to_string()),
                    JobStatus::Cancelling => Ok("Job is being cancelled".to_string()),
                    JobStatus::Cancelled => Ok("Job was cancelled".to_string()),
                },
                None => Ok("Job status not available".to_string()),
            }
        } else {
            Ok("Job summary not available".to_string())
        }
    } else {
        Ok("Job not found".to_string())
    }
}
