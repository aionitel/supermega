mod cli;

use tokio;

#[tokio::main]
async fn main() {
    cli::run().await;
}