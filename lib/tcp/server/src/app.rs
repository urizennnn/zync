use simple_logger::SimpleLogger;
use tcp::tcp::tcp::TCP;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    TCP::run("localhost:8080").await.unwrap();
}
