use super::args::Arguments;

use log::{Level, LevelFilter, info};
use simple_logger::SimpleLogger;

/// Initialize the logger using the settings passed in the command line arguments
pub fn initialize_logging(args: &Arguments)
{
    let mut filter = match args.verbose.log_level().unwrap()
    {
        Level::Error => LevelFilter::Error,
        Level::Warn => LevelFilter::Warn,
        Level::Info => LevelFilter::Info,
        Level::Debug => LevelFilter::Debug,
        Level::Trace => LevelFilter::Trace
    };

    // If this is a debug build, just enable all logging by default
    if cfg!(debug_assertions)
    {
        filter = LevelFilter::Trace;
    }

    SimpleLogger::new().with_level(filter).init().unwrap();

    info!("Logging Initialized");
}
