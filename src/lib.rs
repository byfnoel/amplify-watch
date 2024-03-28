use aws_config::load_from_env;
use aws_sdk_amplify as amplify;
//use thiserror::Error;

#[allow(dead_code, unused_imports, clippy::expect_used)]
#[tokio::main]
async fn main() {
    match establish_connection().await {
        Ok(_) => todo!(),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub async fn establish_connection() -> Result<(), aws_sdk_amplify::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_amplify::Client::new(&config);

    // ... make some calls with the client
    Ok(())
}
