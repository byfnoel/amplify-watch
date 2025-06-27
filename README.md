# Amplify Watch

A simple command-line tool to monitor AWS Amplify app status in real-time.

## Features

- Real-time monitoring of Amplify app build status
- Configurable polling interval

## Prerequisites

- Rust installed on your system
- AWS credentials configured (via AWS CLI, environment variables)
- Access to AWS Amplify service

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd amplify-watch
```

2. Build the project:

```bash
cargo build --release
```

## Usage

### Watch App Status

Monitor an Amplify app in real-time:

```bash
cargo run -- watch --app-id <YOUR_APP_ID> --branch <YOUR_BRANCH_TO_MONITOR>
```

Or with a custom polling interval:

```bash
cargo run -- watch --app-id <YOUR_APP_ID> --branch <YOUR_BRANCH_TO_MONITOR> --interval <INTERVAL_IN_SECONDS(Default:30 seconds)>
```

### Command Options

- `--app-id, -a`: The Amplify app ID to monitor (required)
- `--branch, -a`: The branch to monitor (required)
- `--interval, -i`: Polling interval in seconds (default: 10)

## AWS Configuration

Make sure you have AWS credentials configured. You can do this by:

1. Using AWS CLI: `aws configure`
2. Setting environment variables:
   ```bash
   export AWS_ACCESS_KEY_ID=your_access_key
   export AWS_SECRET_ACCESS_KEY=your_secret_key
   export AWS_REGION=us-east-1
   ```

## LICENSE

Licensed under either of

* Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
