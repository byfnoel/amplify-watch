use aws_config::load_from_env;
use aws_sdk_amplify as amplify;
use aws_sdk_amplify::operation::generate_access_logs::GenerateAccessLogs;
use chrono::{DateTime, Local};

#[::tokio::main]
pub async fn main() -> Result<(), amplify::Error> {
    match establish_connection().await {
        Ok(_) => {
            get_logs(LogsRequestInput {
                start_time: Local::now(),
                end_time: Local::now(),
                domain_name: "example.com".to_string(),
                app_id: "app_id".to_string(),
            })
            .await;
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}

pub async fn establish_connection() -> Result<(), amplify::Error> {
    let config = load_from_env().await;
    let client = amplify::Client::new(&config);
    Ok(())
}

pub struct LogsRequestInput {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub domain_name: String,
    pub app_id: String,
}

pub async fn get_logs(logs_request_input: LogsRequestInput) -> Result<(), GenerateAccessLogs> {
    let config = load_from_env().await;
    let client = amplify::Client::new(&config);

    let request = client
        .generate_access_logs()
        .app_id("app_id")
        .domain_name("domain_name");

    let response = request.send().await;
    Ok(())
}
