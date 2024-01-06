use each::Each;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut e = Each::new(5000);
    e.run().await
}
