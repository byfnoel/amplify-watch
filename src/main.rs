use aws_config::BehaviorVersion;
use aws_sdk_amplify::Client;
use clap::Parser;
use colored::*;
use dotenv::dotenv;
use std::time::Duration;
use tokio::time;

#[derive(Parser)]
#[command(name = "amplify-watch")]
#[command(about = "Monitor AWS Amplify app status in real-time")]
struct Cli {
    /// App ID to monitor
    #[arg(short, long)]
    app_id: String,

    /// Branch name to monitor
    #[arg(short, long, default_value = "main")]
    branch: String,

    /// Polling interval in seconds
    #[arg(short, long, default_value = "10")]
    interval: u64,
}

struct AmplifyMonitor {
    client: Client,
}

impl AmplifyMonitor {
    async fn new() -> Self {
        dotenv().ok();
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Self { client }
    }

    async fn print_realtime_status(&self, app_id: &str, branch: &str, interval: u64) {
        println!(
            "{}",
            "🚀 AWS Amplify Real-Time Status Monitor 🚀"
                .bright_magenta()
                .bold()
        );
        println!("App ID: {}", app_id.bright_magenta());
        println!("Branch: {}", branch.bright_yellow());
        println!("Polling interval: {interval} seconds");
        println!("Press Ctrl+C to stop\n");

        let mut check_count = 0;
        let mut job_counter = 0;
        let max_checks = 20;

        loop {
            check_count += 1;
            println!("Check #{check_count} of {max_checks}");

            if check_count >= max_checks {
                println!("{}", "Maximum checks reached. Stopping...".bright_yellow());
                break;
            }

            let jobs_response = self
                .client
                .list_jobs()
                .app_id(app_id)
                .branch_name(branch)
                .max_results(1)
                .send()
                .await;
            match jobs_response {
                Ok(jobs) => {
                    if let Some(job) = jobs.job_summaries().first() {
                        let status = job.status.as_str();
                        job_counter += 1;
                        println!(
                            "[{}] Job #{} | Status: {}",
                            chrono::Utc::now().format("%H:%M:%S"),
                            job_counter,
                            colorize_status(status)
                        );
                    } else {
                        println!("No jobs found for this branch.");
                    }
                }
                Err(e) => {
                    println!("Error fetching jobs: {e}");
                }
            }
            time::sleep(Duration::from_secs(interval)).await;
        }
    }
}

fn colorize_status(status: &str) -> colored::ColoredString {
    match status.to_lowercase().as_str() {
        "succeeded" | "success" => status.green().bold(),
        "failed" | "error" => status.red().bold(),
        "in_progress" | "running" => status.yellow().bold(),
        "pending" | "waiting" => status.blue().bold(),
        "cancelled" => status.magenta().bold(),
        _ => status.white(),
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let monitor = AmplifyMonitor::new().await;
    monitor
        .print_realtime_status(&cli.app_id, &cli.branch, cli.interval)
        .await;
}
