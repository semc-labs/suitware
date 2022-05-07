use color_eyre::eyre::Result;
use tracing::instrument;

use actix::prelude::*;

use suitware_diagnostics::{basic::Basic, Ping};

mod setup;

#[instrument]
#[actix_rt::main]
async fn main() -> Result<()> {
    setup::setup_debugging()?;

    let basic = Basic {}.start();
    basic.send(Ping {}).await?;

    System::current().stop();
    Ok(())
}
