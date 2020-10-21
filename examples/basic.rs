#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = stockquote::get("IBM").await?;
    println!("{:?}", result);
    Ok(())
}


