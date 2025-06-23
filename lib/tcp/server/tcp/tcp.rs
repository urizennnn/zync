use super::allowed_request::AllowedRequest;
use crate::http::get::get_file;
use crate::http::methods::list;
use crate::http::put::router;
use crate::threadpool::thread::Threadpool;
use log::{error, info, warn};
use std::error::Error;
use std::process::exit;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 8 * 1024; // 8KB

pub struct TCP;

impl TCP {
    pub fn accept_connection_sync(
        addr: &str,
        global_rt: &tokio::runtime::Runtime,
    ) -> Result<(), Box<dyn Error>> {
        let socket_addr: std::net::SocketAddr = addr.parse()?;
        global_rt.spawn(async move {
            warp::serve(router()).run(socket_addr).await;
        });
        Ok(())
    }

    /// Runs the TCP server, accepting incoming connections and handling each client concurrently.
    ///
    /// Binds to the specified address, listens for incoming TCP connections, and processes each client using a thread pool and asynchronous tasks. Logs server status and connection errors.
    ///
    /// # Parameters
    /// - `addr`: The address to bind the TCP server to (e.g., "127.0.0.1:8080").
    ///
    /// # Returns
    /// `Ok(())` if the server runs without binding errors; otherwise, returns an error if binding fails. The function runs indefinitely unless a critical error occurs during thread pool creation.
    pub async fn run(addr: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        info!("Server listening on {}", addr);

        let pool = Threadpool::build(6).unwrap_or_else(|_| {
            error!("Failed to create thread pool");
            exit(1);
        });

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    pool.execute(|| {
                        tokio::spawn(async move {
                            if let Err(e) = TCP::handle_client(stream).await {
                                error!("Error handling client: {}", e);
                            }
                        });
                    });
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
    }

    /// Handles a single TCP client connection asynchronously.
    ///
    /// Reads requests from the client, parses them, and processes supported commands such as listing storage or retrieving files. Logs and handles unknown or unsupported requests. The connection is closed when the client disconnects.
    ///
    /// # Parameters
    /// - `stream`: The TCP stream representing the client connection.
    ///
    /// # Returns
    /// `Ok(())` if the client was handled successfully, or an error if an I/O or protocol error occurs.
    async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = vec![0u8; BUFFER_SIZE];
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                warn!("Connection closed by client");
                break;
            }
            let request = String::from_utf8_lossy(&buffer[..n])
                .trim_matches(char::from(0))
                .trim()
                .to_string();
            info!("Received request: {}", request);

            match AllowedRequest::from_str_slice(&request) {
                Some(AllowedRequest::List) => {
                    list::list_storage(&mut stream).await?;
                }
                Some(AllowedRequest::Delete) => {
                    info!("Processing DELETE request");
                }
                Some(AllowedRequest::Get) => {
                    get_file(&mut stream, &mut buffer).await?;
                }
                None => warn!("Unknown request: {request}"),
                _ => {}
            }

            stream.flush().await?;
        }
        Ok(())
    }
}
