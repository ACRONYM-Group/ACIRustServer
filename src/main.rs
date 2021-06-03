#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    if let Ok(rt) = tokio::runtime::Runtime::new()
    {
        if opt.raw_socket
        {
            log::info!("Using Raw Socket Server");

            if let Err(e) = rt.block_on(async {socket::run(opt).await})
            {
                eprintln!("ACI Raw Socket Server encountered an error: {}", e)
            }
        }
        else
        {
            log::info!("Using Web Socket Server");

            if let Err(e) = rt.block_on(async {connect::run(opt).await})
            {
                eprintln!("ACI Web Socket Server encountered an error: {}", e)
            }
        }
    }
    else
    {
        log::error!("Unable to start a Tokio Runtime")
    }
}