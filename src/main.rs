#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let server = TcpListener::bind("127.0.0.1:8765").unwrap();

    log::info!("Starting server!");

    let aci = std::sync::Arc::new(server::Server::new(&opt).unwrap());

    for stream in server.incoming()
    {
        let mut conn = server::ServerInterface::new(&aci.clone());
        spawn (move ||
            {
            let mut websocket = accept(stream.unwrap()).unwrap();

            log::info!("Got a connection!");

            loop
            {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text()
                {
                    let text = msg.clone().into_text().unwrap();
                    log::info!("Recieved Message: `{}`", text);

                    let result = conn.execute_command(commands::Command::from_string(&text).unwrap()).unwrap();

                    if result.is_some()
                    {
                        let msg = result.unwrap();
                        log::info!("Response: {}", msg);
                        websocket.write_message(tungstenite::Message::Text(msg.to_string())).unwrap();
                    }
                }
            }
        });
    }
}
