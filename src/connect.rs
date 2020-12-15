use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};

use super::args;
use super::server;
use super::commands;

use tokio::sync::Mutex;

use serde_json::json;

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
        tokio::spawn(handle_stream(stream, aci.clone()));
    }

    Ok(())
}

pub async fn handle_stream(stream: TcpStream, aci: std::sync::Arc<server::Server>)
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
                    tokio::spawn(handle_message(tx.clone(), val, interface.clone()));
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

async fn handle_message(tx: std::sync::Arc<tokio::sync::mpsc::UnboundedSender<std::result::Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>>>, val: serde_json::Value, aci_interface: std::sync::Arc<Mutex<server::ServerInterface>>)
{
    if let Ok(command) = commands::Command::from_json(val)
    {
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
                    "UNKNOWN".to_string()
                }
            },
            Err(e) =>
            {
                "UNKNOWN".to_string()
            }
        };

        log::info!("Sending json back {:?}", json_msg);
        tx.send(Ok(tokio_tungstenite::tungstenite::Message::Text(json_msg))).unwrap();
    }
}