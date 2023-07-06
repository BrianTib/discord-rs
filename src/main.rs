#[allow(dead_code)]
mod example;

#[tokio::main]
async fn main() {
    example::main().await;
}