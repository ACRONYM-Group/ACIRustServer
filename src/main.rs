#![allow(dead_code)]

mod args;
mod logging;
mod commands;

use structopt::StructOpt;
use log::error;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let r = commands::Command::from_string("{}");

    match r
    {
        Err(e) => error!("{:?}", e),
        _ => {}
    }
}
