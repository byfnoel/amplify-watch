use aws_config::load_from_env;
use aws_sdk_amplify as amplify;
use aws_sdk_amplify::operation::generate_access_logs::GenerateAccessLogs;
use chrono::{DateTime, Local};

#[tokio::main]
async fn main() {
    match establish_connection().await {
        Ok(_) => todo!(),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub async fn establish_connection() -> Result<(), aws_sdk_amplify::Error> {
    let config = load_from_env().await;
    let client = amplify::Client::new(&config);

    // make the call to `get_logs` here
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
