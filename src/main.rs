#![allow(dead_code)]

mod args;
mod logging;
mod commands;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);
}
