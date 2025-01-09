use std::error::Error;

use zync::{init::init_app, utils::poll::poll_future};

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(init_app())
}
