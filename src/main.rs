use std::error::Error;
use zync::init::init_app;

fn main() -> Result<(), Box<dyn Error>> {
    init_app()
}
