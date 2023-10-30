use server::start;

#[tokio::main]
async fn main() {
    start::start().await;
}
