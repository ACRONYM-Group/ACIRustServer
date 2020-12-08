//! Client Command Data
use serde_json::Value;
use log::{trace, error};

use super::verify_command;

/// Commands available from the client
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Commands
{
    WriteToDisk,
    ReadFromDisk,
    ListDatabases,
    GetValue,
    SetValue,
    GetIndex,
    SetIndex,
    AppendIndex,
    GetLengthIndex,
    GetRecentIndex,
    CreateDatabase,
    AcronymAuth,
    GoogleAuth,
    Event
}

/// Errors from parsing
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommandParsingError
{
    BadJSON(String),
    BadPacket(String),
    ArgumentsNotPresent(String)
}

/// Holds the arguments of a command
#[derive(Debug, Clone)]
pub struct Command
{
    pub cmd: Commands,    
    pub data: Value
}

impl Command
{
    /// Generate a command from a string
    pub fn from_string(data: &str) -> Result<Command, CommandParsingError>
    {
        trace!("Parsing string packet {:?}", data);

        match serde_json::from_str(data)
        {
            Ok(value) => Command::from_json(value),
            Err(error) => 
            {
                let msg = format!("Bad JSON Packet {:?}", error);
                error!("{}", msg);
                Err(CommandParsingError::BadJSON(msg))
            }
        }   
    }

    /// Generate a command from a json value
    pub fn from_json(value: Value) -> Result<Command, CommandParsingError>
    {
        trace!("Parsing JSON packet {:?}", value);

        match value
        {
            Value::Object(data) =>
            {
                if let Some(cmd) = data.get("cmdType")
                {
                    if let Value::String(cmd_str) = cmd
                    {
                        let rest_value = Value::Object(data.clone());
                        
                        let cmd_type = match cmd_str.as_str()
                        {
                            "wtd" => Commands::WriteToDisk,
                            "rfd" => Commands::ReadFromDisk,
                            "list_databases" => Commands::ListDatabases,
                            "get_val" => Commands::GetValue,
                            "set_val" => Commands::SetValue,
                            "get_index" => Commands::GetIndex,
                            "set_index" => Commands::SetIndex,
                            "append_index" => Commands::AppendIndex,
                            "get_len_index" => Commands::GetLengthIndex,
                            "get_recent_index" => Commands::GetRecentIndex,
                            "cdb" => Commands::CreateDatabase,
                            "a_auth" => Commands::AcronymAuth,
                            "g_auth" => Commands::GoogleAuth,
                            "event" => Commands::Event,
                            _ => 
                            {
                                let msg = format!("cmdType field of an unknown type {:?}", cmd);
                                error!("{}", msg);
                                return Err(CommandParsingError::BadPacket(msg));
                            }
                        };

                        let command = Command {data: rest_value, cmd: cmd_type};
                        verify_command(&command)?;
                        Ok(command)
                    }
                    else
                    {
                        let msg = format!("cmdType field not a string, got {:?}", cmd);
                        error!("{}", msg);
                        Err(CommandParsingError::BadPacket(msg))
                    }
                }
                else
                {
                    let msg = "No cmdType field given".to_string();
                    error!("{}", msg);
                    Err(CommandParsingError::BadPacket(msg))
                }
            },
            default => 
            {
                let msg = format!("Parsed data is not an object, got {:?}", default);
                error!("{}", msg);
                Err(CommandParsingError::BadPacket(msg))
            }
        }
    }
}
