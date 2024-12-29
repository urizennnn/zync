use std::error::Error;

use zync::init::init_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_app()
}
