#![allow(dead_code)]

mod args;
mod logging;
mod commands;
mod database;

use structopt::StructOpt;

static BUILD_VERSION: &str = "dev2020.12.05.1";

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);
}
