use color_eyre::eyre::Result;
use tracing::{error, instrument};

use suitware_util::ok;

/// Set up debugging.
#[instrument]
pub fn setup_debugging() -> Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    // Set up hooks.
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();
    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        error!("{}", panic_hook.panic_report(pi));
    }));

    // Set up Tracing.

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("trace"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    ok!("debugging");

    Ok(())
}
