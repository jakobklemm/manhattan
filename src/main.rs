use manhattan::Error;

use tracing::info;
use tracing_subscriber;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    info!("Hello, world!");

    Ok(())
}
