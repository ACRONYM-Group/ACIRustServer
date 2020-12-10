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
                log::info!("Got a connection!");

                if stream.is_ok()
                {
                    match accept(stream.unwrap())
                    {
                        Ok(mut websocket) => 
                        {
                            loop
                            {
                                match websocket.read_message()
                                {
                                    Ok(msg) =>
                                    {
                                        if msg.is_text()
                                        {
                                            // This is ok because it is checked in the above if statement
                                            let text = msg.clone().into_text().unwrap();
                                            log::info!("Recieved Message: `{}`", text);

                                            match serde_json::from_str(&text)
                                            {
                                                Ok(data) =>
                                                {
                                                    let data: serde_json::Value = data;
                                                    match commands::Command::from_json(data.clone())
                                                    {
                                                        Ok(command) =>
                                                        {
                                                            match conn.execute_command(command)
                                                            {
                                                                Ok(result) =>
                                                                {
                                                                    if result.is_some()
                                                                    {
                                                                        // This is ok because it is checked again
                                                                        let msg = result.unwrap();
                                                                        log::info!("Response: {}", msg);
                                                                        websocket.write_message(tungstenite::Message::Text(msg.to_string())).unwrap();
                                                                    }
                                                                    else
                                                                    {
                                                                        log::warn!("Not sending response per server interface responding with None");
                                                                    }
                                                                },
                                                                Err(e) =>
                                                                {
                                                                    let cmd = if let serde_json::Value::Object(data) = data
                                                                    {
                                                                        if data.contains_key("cmd")
                                                                        {
                                                                            data.get("cmd").unwrap().clone()
                                                                        }
                                                                        else
                                                                        {
                                                                            serde_json::json!("unknown")
                                                                        }
                                                                    }
                                                                    else
                                                                    {
                                                                        serde_json::json!("unknown")
                                                                    };

                                                                    let msg = serde_json::json!({"cmd": cmd, "mode": "error", "msg": format!("{}", e)});
                                                                    websocket.write_message(tungstenite::Message::Text(msg.to_string())).unwrap();
                                                                }
                                                            }
                                                        },
                                                        Err(e) =>
                                                        {
                                                            let cmd = if let serde_json::Value::Object(data) = data
                                                            {
                                                                if data.contains_key("cmd")
                                                                {
                                                                    data.get("cmd").unwrap().clone()
                                                                }
                                                                else
                                                                {
                                                                    serde_json::json!("unknown")
                                                                }
                                                            }
                                                            else
                                                            {
                                                                serde_json::json!("unknown")
                                                            };

                                                            let msg = serde_json::json!({"cmd": cmd, "mode": "error", "msg": format!("{:?}", e)});
                                                            websocket.write_message(tungstenite::Message::Text(msg.to_string())).unwrap();
                                                        }
                                                    }
                                                },
                                                Err(e) =>
                                                {
                                                    log::error!("Unable to parse message from json, {}", e);
                                                }
                                            }
                                        }
                                    },
                                    Err(e) =>
                                    {
                                        log::error!("Unable to parse websocket, {}", e);
                                    }
                                }
                            }
                        },
                        Err(e) =>
                        {
                            log::error!("Unable to accept stream, {}", e);
                        }
                    }
                }
                else
                {
                    log::error!("Stream is not able to be unwraped, {}", stream.unwrap_err());
                }
        });
    }
}
