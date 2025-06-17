use reqwest;

pub async fn mock_crosschain_transfer(tx: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3000/api")
        .query(&[("module", "proxy"), ("action", "eth_sendRawTransaction"), ("tx", &tx)])
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn estimate_latency() -> f64 {
    let start = std::time::Instant::now();
    let _ = reqwest::get("https://api.etherscan.io/api").await;
    start.elapsed().as_secs_f64()
}