use aws_sdk_amplify::Client;
use colored::*;
use dotenv::dotenv;
use std::{error::Error, time::Duration};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    println!(
        "{}",
        "🚀 AWS Amplify Real-Time Status Monitor 🚀"
            .bright_magenta()
            .bold()
    );

    let app_id = std::env::var("AMPLIFY_APP_ID")
        .unwrap_or_else(|_| panic!("AMPLIFY_APP_ID must be set in the environment"));
    let branch_name = std::env::var("AMPLIFY_BRANCH_NAME").unwrap_or_else(|_| "master".to_string());

    loop {
        match client
            .list_jobs()
            .app_id(&app_id)
            .branch_name(&branch_name)
            .send()
            .await
        {
            Ok(response) => {
                for job_summary in &response.job_summaries {
                    println!(
                        "   Job id  {:#?} =>  {:#?}",
                        job_summary.job_id(),
                        job_summary.status()
                    );
                }
            }
            Err(e) => eprintln!("Error fetching jobs: {}", e),
        }

        time::sleep(Duration::from_secs(8)).await;
    }
}
