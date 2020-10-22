# StockQuote

A safe and easy to use stock quote library. Based on Yahoo Finance.

Usage:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = stockquote::get("IBM").await?;
    println!("{:?}", result);
    Ok(())
}
```

## Upcoming

- [ ] Price history data
- [ ] Realtime price data
