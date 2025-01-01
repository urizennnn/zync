use lib_tcp::{
    init::{init, update_init},
    methods::{get::receive_files, upload::upload},
};
use log::{error, info, warn};
use once_cell::sync::Lazy;
use simple_logger::SimpleLogger;
use std::{
    error::Error,
    io::{self, BufRead, Write},
};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};
use whoami::username;

pub static USER: Lazy<String> = Lazy::new(|| username().to_string());

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();
    let mut stream = TcpStream::connect("localhost:8080").await?;
    stream.write_all(USER.as_bytes()).await?;
    stream.flush().await?;
    info!("Connected to server");
    init().await?;
    let mut buffer = vec![0; 5_242_880];

    loop {
        print!(
            "Enter command (LIST, GET <filename>, PUT <filename>, DELETE <filename>, or EXIT): "
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let input = input.trim().to_string();

        match input.split_whitespace().next() {
            Some("EXIT") => {
                info!("Exiting");
                break;
            }
            Some("PUT") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    error!("Invalid PUT command. Usage: PUT <filename>");
                    continue;
                }
                stream.write_all(input.as_bytes()).await?;
                stream.flush().await?;
                update_init(parts[1].to_string()).await?;
                upload(&mut stream, parts[1], &mut buffer).await?;
            }
            Some("LIST") => {
                stream.write_all(input.as_bytes()).await?;
                stream.flush().await?;
                list(&mut stream, &mut buffer).await?;
            }
            Some("GET") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                let dest: Vec<&str> = input.split_whitespace().collect();

                if parts.len() != 3 {
                    error!("Invalid GET command. Usage: GET <filename> <destination-path>");
                    continue;
                }
                stream.write_all(input.as_bytes()).await?;
                stream.flush().await?;

                // Read server response
                let n = stream.read(&mut buffer).await?;
                let response = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                info!("Received response: {}", response);

                if response.starts_with("SEND") {
                    let parts: Vec<&str> = response.split_whitespace().collect();
                    receive_files(&mut stream, &mut buffer, &parts, &dest).await?;
                } else {
                    error!("File not found on server");
                }
            }
            Some("DELETE") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    error!("Invalid DELETE command. Usage: DELETE <filename>");
                    continue;
                }
                stream.write_all(input.as_bytes()).await?;
                stream.flush().await?;

                // Read server response
                let n = stream.read(&mut buffer).await?;
                let response = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                info!("Received response: {}", response);

                // Add DELETE functionality here
            }
            _ => {
                warn!("Invalid command. Please try again.");
            }
        }
    }
    Ok(())
}
