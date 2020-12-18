#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    if let Ok(rt) = tokio::runtime::Runtime::new()
    {
        match rt.block_on(async {connect::run(opt).await})
        {
            Ok(_) => {},
            Err(e) => eprintln!("ACI Server encountered an error: {}", e)
        }
    }
    else
    {
        log::error!("Unable to start a Tokio Runtime")
    }
}