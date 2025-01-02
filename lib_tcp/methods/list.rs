use log::{info, warn};
use serde_json::Value;
use std::{error::Error, str};
use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn list(stream: &mut TcpStream, buf: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let request = stream.read(buf).await?;
    let initial_message = str::from_utf8(&buf[..request])?.trim();
    info!("{}", initial_message);

    let request = stream.read(buf).await?;
    let json_response = str::from_utf8(&buf[..request])?.trim();

    let file_list: Value = serde_json::from_str(json_response)?;
    if let Value::Array(files) = file_list {
        for file in files {
            println!("{}", file.as_str().unwrap_or(""));
        }
    } else {
        warn!("Unexpected response format");
    }

    Ok(())
}
