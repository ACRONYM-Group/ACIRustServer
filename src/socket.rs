use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::OwnedWriteHalf;

use super::args;
use super::server;
use super::commands;

use tokio::sync::Mutex;

use serde_json::json;

use chashmap::CHashMap;

type SendingChannel = tokio::sync::mpsc::Sender<std::string::String>;

pub async fn run(opt: args::Arguments, aci: std::sync::Arc<server::Server>) -> Result<(), String>
{
    log::info!("Starting Raw ACI Server");

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

    // Testing port
    let port = 8766;

    // FIXME: Correct this, this is stupid and only for testing!
    
    /*if opt.ignore_config || opt.port.is_some()
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
    };*/ 

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

fn print_type_of<T>(_: &T)
{
    println!("{}", std::any::type_name::<T>())
}

pub async fn handle_stream(mut stream: TcpStream, aci: std::sync::Arc<server::Server>, connections_hashmap: std::sync::Arc<CHashMap<String, SendingChannel>>)
{
    let interface = std::sync::Arc::new(Mutex::new(server::ServerInterface::new(&aci)));

    let addr = if let Ok(s) = stream.peer_addr() {s.to_string()} else {"UNKNOWN".to_string()};

    log::info!("Got peer connection from '{}'", addr);

    let (mut rx, mut tx) = stream.into_split();
    
    let (mut msg_sender, mut msg_reciever ) = tokio::sync::mpsc::channel::<String>(128);

    tokio::spawn(async move {
        while let Some(msg) = msg_reciever.recv().await
        {
            tx.write(msg.as_bytes()).await;
        }
    });

    loop
    {
        let mut buffer = String::new();
        if let Err(e) = rx.read_to_string(&mut buffer).await
        {
            log::error!("Unable to read from connection from `{}`: `{}`", addr, e);
        }

        if let Ok(val) = serde_json::from_str(&buffer)
        {
            let val: serde_json::Value = val;

            handle_message(msg_sender.clone(), val, interface.clone(), connections_hashmap.clone()).await;
        }
        else
        {

        }
    }
}

async fn handle_message(tx: SendingChannel, val: serde_json::Value, aci_interface: std::sync::Arc<Mutex<server::ServerInterface>>, connections_hashmap: std::sync::Arc<CHashMap<String, SendingChannel>>)
{
    match val
    {
        serde_json::Value::Object(obj) => 
        {
            let json = handle_individual(tx.clone(), serde_json::Value::Object(obj), aci_interface, connections_hashmap).await;

            if let Ok(Some(json)) = json
            {
                if let Err(e) = tx.send(json.to_string()).await
                {
                    log::error!("Unable to send message, got error: `{}`", e);
                    return;
                }
            }
        },
        serde_json::Value::Array(values) => 
        {
            let mut result = vec![];
            for value in values
            {
                let json = handle_individual(tx.clone(), value, aci_interface.clone(), connections_hashmap.clone()).await;

                if let Ok(Some(json)) = json
                {
                    result.push(json);
                }
            }

            if result.len() > 0
            {
                if let Err(e) = tx.send(serde_json::json!(result).to_string()).await
                {
                    log::error!("Unable to send message, got error: `{}`", e);
                    return;
                }
            }
        },
        default =>
        {
            log::error!("Unable to handle a value which is not an object or array, got {:?}", default);
        }
    }
}

async fn handle_individual(tx: SendingChannel, val: serde_json::Value, aci_interface: std::sync::Arc<Mutex<server::ServerInterface>>, connections_hashmap: std::sync::Arc<CHashMap<String, SendingChannel>>) -> Result<Option<serde_json::Value>, ()>
{
    let no_ack = if let serde_json::Value::Object(map) = &val
    {
        if let Some(serde_json::Value::Bool(b)) = map.get("no_ack")
        {
            b.clone()
        }
        else
        {
            false
        }
    }
    else
    {
        false
    };

    if no_ack
    {
        log::debug!("Not acknowledging per `no_ack`");
    }

    if let Ok(command) = commands::Command::from_json(val.clone())
    {
        if command.cmd == commands::Commands::Event
        {
            if let serde_json::Value::Object(map) = command.data
            {
                if let Some(serde_json::Value::String(dest)) = map.get("destination")
                {
                    if let Some(conn) = connections_hashmap.get(dest)
                    {
                        if let Err(e) = tx.send(val.to_string()).await
                        {
                            log::error!("Unable to send message, got error: `{}`", e);
                            return Err(());
                        }

                        if no_ack
                        {
                            return Ok(None);
                        }
                        {   
                            if let (Some(event_id), Some(origin)) = (map.get("event_id"), map.get("origin"))
                            {
                                return Ok(Some(serde_json::json!({"cmd": "event", "mode": "ack", "event_id": event_id, "origin": origin})));
                            }
                        }   
                    }
                    else
                    {
                        log::warn!("Attempted to forward event to `{}`, however, this user is not connected", dest);
                        let msg = format!("Unable to connect to user `{}`", dest);
                        
                        if no_ack
                        {
                            return Ok(None);
                        }
                        {   
                            if let (Some(event_id), Some(origin)) = (map.get("event_id"), map.get("origin"))
                            {
                            return Ok(Some(serde_json::json!({"cmd": "event", "mode": "error", "msg": msg, "event_id": event_id, "origin": origin})));
                            }
                        } 
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

            return Err(());
        }

        let is_auth_command = command.cmd == commands::Commands::AcronymAuth || command.cmd == commands::Commands::GoogleAuth;

        let result = aci_interface.lock().await.execute_command(command);

        let json_msg = match result
        {
            Ok(val) =>
            {
                if let Some(val) = val
                {
                    val
                }
                else
                {
                    return Err(());
                }
            },
            Err(e) =>
            {
                json!({"cmd": "UNKNOWN", "mode": "error", "msg": format!("Error from server process: {}", e)})
            }
        };

        log::debug!("Sending data back {:?}", json_msg);
        
        if is_auth_command
        {
            let id = aci_interface.lock().await.user_profile.name.clone();
            connections_hashmap.insert(id.clone(), tx.clone());

            log::debug!("Adding Connection to user `{}` to connections map", id);
        }

        if no_ack
        {
            return Ok(None);
        }
        else
        {
            return Ok(Some(json_msg));
        }
    }

    Err(())
}