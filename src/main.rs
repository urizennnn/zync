use app::init_app;
use std::error::Error;

mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_app()
}
