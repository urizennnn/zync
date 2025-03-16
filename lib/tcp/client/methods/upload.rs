use bytes::{BufMut, BytesMut};
use futures_util::sink::SinkExt;
use std::error::Error;
use tokio::{fs, net::TcpStream};
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

pub async fn upload(stream: &mut TcpStream, path: &str) -> Result<(), Box<dyn Error>> {
    // Ensure the file path is not empty.
    if path.trim().is_empty() {
        return Err("Provided file path is empty".into());
    }

    // Read the file from disk.
    let file_bytes = fs::read(path).await?;
    if file_bytes.is_empty() {
        return Err(format!("File '{}' is empty", path).into());
    }

    // Build the payload: [filename + delimiter (0) + file contents]
    let mut payload = BytesMut::new();
    payload.put(path.as_bytes());
    payload.put_u8(0); // delimiter separating filename and file bytes
    payload.put(file_bytes.as_slice());

    // Wrap the stream in a length-delimited framed writer.
    let mut framed = FramedWrite::new(stream, LengthDelimitedCodec::new());

    // Send the entire payload as one frame.
    framed.send(payload.freeze()).await?;

    // Flush the stream to ensure all data is sent.
    framed.flush().await?;

    Ok(())
}
