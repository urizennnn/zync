use reqwest::Client;
use std::error::Error;
use std::path::Path;
use tokio::fs;

pub async fn upload(file_path: &str, url: &str) -> Result<(), Box<dyn Error>> {
    if file_path.trim().is_empty() {
        return Err("Provided file path is empty".into());
    }

    let file_bytes = fs::read(file_path).await?;
    if file_bytes.is_empty() {
        return Err(format!("File '{}' is empty", file_path).into());
    }

    let filename = Path::new(file_path)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or(file_path);

    let request_url = reqwest::Url::parse_with_params(url, &[("path", filename)])?;

    let client = Client::new();
    let response = client.post(request_url).body(file_bytes).send().await?;

    if !response.status().is_success() {
        panic!("failed {:?}", response.error_for_status());
        return Err(format!("HTTP request failed with status: {}", response.status()).into());
    }

    Ok(())
}
