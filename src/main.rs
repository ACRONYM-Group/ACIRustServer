#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);
}
