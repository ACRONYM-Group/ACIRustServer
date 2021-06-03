#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

async fn run_both(aci: std::sync::Arc<server::Server>, opt: args::Arguments)
{
    let aci0 = aci.clone();
    let aci1 = aci.clone();

    let opt0 = opt.clone();
    let opt1 = opt.clone();

    let raw_socket = tokio::spawn(async move {socket::run(opt0, aci0).await});
    let web_socket = tokio::spawn(async move {connect::run(opt1, aci1).await});

    tokio::join!(raw_socket, web_socket);
}


fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    log::info!("Loading ACI Instance");

    match server::Server::new(&opt)
    {
        Ok(aci) =>
        {
            let aci = std::sync::Arc::new(aci);

            if let Ok(rt) = tokio::runtime::Runtime::new()
            {
                if opt.raw_socket
                {
                    log::info!("Using Raw Socket Server");

                    if let Err(e) = rt.block_on(async {socket::run(opt, aci.clone()).await})
                    {
                        eprintln!("ACI Raw Socket Server encountered an error: {}", e)
                    }
                }
                else if opt.both
                {
                    log::info!("Using Both Raw Socket and Web Socket");

                    rt.block_on(async {run_both(aci, opt).await});
                }
                else
                {
                    log::info!("Using Web Socket Server");

                    if let Err(e) = rt.block_on(async {connect::run(opt, aci.clone()).await})
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
        Err(e) =>
        {
            eprintln!("Unable to load ACI Server, got error {}", e);
        }
    }
}