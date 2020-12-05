#![allow(dead_code)]

mod args;
mod logging;
mod commands;
mod database;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);
}
