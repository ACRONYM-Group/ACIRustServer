use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};

use super::args;
use super::server;
use super::commands;

use tokio::sync::Mutex;

use serde_json::json;

use chashmap::CHashMap;

type SendingChannel = std::sync::Arc<tokio::sync::mpsc::UnboundedSender<std::result::Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>>>;

pub async fn run(opt: args::Arguments) -> Result<(), String>
{
    log::info!("Starting ACI Server");
    let aci = std::sync::Arc::new(server::Server::new(&opt)?);

    let ip = if opt.ignore_config || opt.ip.is_some()
    {
        match opt.ip
        {
            Some(ip) => ip,
            None =>
            {
                let msg = "IP must be provided if the --ignore-config (-i) flag is passed".to_string();
                log::error!("{}", msg);
                return Err(msg);
            }
        }
    }
    else
    {
        aci.config_get_ip()?
    };

    let port = if opt.ignore_config || opt.port.is_some()
    {
        match opt.port
        {
            Some(port) => port,
            None =>
            {
                let msg = "Port must be provided if the --ignore-config (-i) flag is passed".to_string();
                log::error!("{}", msg);
                return Err(msg);
            }
        }
    }
    else
    {
        aci.config_get_port()?
    };

    let connections_hashmap: std::sync::Arc<CHashMap<String, SendingChannel>> = std::sync::Arc::new(CHashMap::new());

    let addr = format!("{}:{}", ip, port);
    log::info!("Connecting to address `{}`", addr);

    // Try to connect
    let conn = if let Ok(conn) = TcpListener::bind(&addr).await {conn} else
    {
        let msg = format!("Unable to bind listener to `{}`", addr);
        log::error!("{}", msg);
        return Err(msg);
    };

    // Reading loop
    while let Ok((stream, _)) = conn.accept().await
    {
        tokio::spawn(handle_stream(stream, aci.clone(), connections_hashmap.clone()));
    }

    Ok(())
}

pub async fn handle_stream(stream: TcpStream, aci: std::sync::Arc<server::Server>, connections_hashmap: std::sync::Arc<CHashMap<String, SendingChannel>>)
{
    let interface = std::sync::Arc::new(Mutex::new(server::ServerInterface::new(&aci)));

    let addr = if let Ok(s) = stream.peer_addr() {s.to_string()} else {"UNKNOWN".to_string()};

    log::info!("Got peer connection from '{}'", addr);

    let ws_stream = if let Ok(s) = tokio_tungstenite::accept_async(stream).await {s} else
    {
        let msg = format!("Unable to open websocket with peer at `{}`", addr);
        log::error!("{}", msg);
        return;
    };

    let (tx, rx) = ws_stream.split();

    let (stx, srx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(srx.forward(tx));

    let tx = std::sync::Arc::new(stx);

    rx.for_each(|msg| async
    {
        if let Ok(msg) = msg
        {
            if let Ok(text) = msg.to_text()
            {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(text)
                {
                    tokio::spawn(handle_message(tx.clone(), val, interface.clone(), connections_hashmap.clone()));
                }
                else
                {
                    log::error!("Unable to parse json from message from `{}`", addr);
                }
            }
            else
            {
                log::error!("Unable to extract text from message from `{}`", addr);
            }
        }
        else
        {
            log::error!("Unable to extract message from `{}`", addr);
        }
    }).await;
}

async fn handle_message(tx: SendingChannel, val: serde_json::Value, aci_interface: std::sync::Arc<Mutex<server::ServerInterface>>, connections_hashmap: std::sync::Arc<CHashMap<String, SendingChannel>>)
{
    if let Ok(command) = commands::Command::from_json(val.clone())
    {
        if command.cmd == commands::Commands::Event
        {
            if let serde_json::Value::Object(map) = command.data
            {
                if map.contains_key("destination")
                {
                    if let serde_json::Value::String(dest) = map.get("destination").unwrap()
                    {
                        if connections_hashmap.contains_key(dest)
                        {
                            connections_hashmap.get(dest).unwrap().send(Ok(tokio_tungstenite::tungstenite::Message::Text(
                                val.to_string()
                            ))).unwrap();

                            tx.send(Ok(tokio_tungstenite::tungstenite::Message::Text(
                                serde_json::json!({"cmd": "event", "mode": "ack", "event_id": map.get("event_id").unwrap(), "origin": map.get("origin").unwrap()}).to_string()
                            ))).unwrap();
                        }
                        else
                        {
                            log::warn!("Attempted to forward event to `{}`, however, this user is not connected", dest);
                            let msg = format!("Unable to connect to user `{}`", dest);
                            tx.send(Ok(tokio_tungstenite::tungstenite::Message::Text(
                                serde_json::json!({"cmd": "event", "mode": "error", "msg": msg, "event_id": map.get("event_id").unwrap(), "origin": map.get("origin").unwrap()}).to_string()
                            ))).unwrap();
                        }
                    }
                    else
                    {
                        log::error!("Event destination is not a string, ignoring packet");
                    }
                }
                else
                {
                    log::error!("Event command does not include destination, ignoring packet");
                }
            }
            else
            {
                log::error!("Command data is not an object, ignoring packet");
            }

            return;
        }

        let is_auth_command = command.cmd == commands::Commands::AcronymAuth || command.cmd == commands::Commands::GoogleAuth;

        let result = aci_interface.lock().await.execute_command(command);

        let json_msg = match result
        {
            Ok(val) =>
            {
                if let Some(val) = val
                {
                    val.to_string()
                }
                else
                {
                    return;
                }
            },
            Err(e) =>
            {
                json!({"cmd": "UNKNOWN", "mode": "error", "msg": format!("Error from server process: {}", e)}).to_string()
            }
        };

        log::debug!("Sending data back {:?}", json_msg);
        tx.send(Ok(tokio_tungstenite::tungstenite::Message::Text(json_msg))).unwrap();

        if is_auth_command
        {
            let id = aci_interface.lock().await.user_profile.name.clone();
            connections_hashmap.insert(id.clone(), tx.clone());

            log::debug!("Adding Connection to user `{}` to connections map", id);
        }
    }
}