use rskafka::client::ClientBuilder;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Attempting connect...");
    let connection = "0.0.0.0:9092".to_string();
    let client = ClientBuilder::new(vec![connection]).build().await.unwrap();
    println!("Hello, world!");
}
