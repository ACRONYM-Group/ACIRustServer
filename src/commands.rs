//! Client Command Data
use serde_json::Value;
use log::{info, trace};

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
    cmd: Commands,
    
    data: Value
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
                let msg = format!("{:?}", error);
                info!("{}", msg);
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
                                let msg = format!("cmdType field not a string, got {:?}", cmd);
                                info!("{}", msg);
                                return Err(CommandParsingError::BadPacket(msg));
                            }
                        };

                        Ok(Command
                        {
                            data: rest_value,
                            cmd: cmd_type
                        })
                    }
                    else
                    {
                        let msg = format!("cmdType field not a string, got {:?}", cmd);
                        info!("{}", msg);
                        Err(CommandParsingError::BadPacket(msg))
                    }
                }
                else
                {
                    let msg = "No cmdType field given".to_string();
                    info!("{}", msg);
                    Err(CommandParsingError::BadPacket(msg))
                }
            },
            default => 
            {
                let msg = format!("Parsed data is not an object, got {:?}", default);
                info!("{}", msg);
                Err(CommandParsingError::BadPacket(msg))
            }
        }
    }
}

fn test_command_parsing(test_output: bool)
{
    let examples = vec![
        "{\"cmdType\": \"wtd\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"rfd\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"list_databases\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"get_val\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"set_val\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"get_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"index\":42}",
        "{\"cmdType\": \"set_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\", \"index\":42}",
        "{\"cmdType\": \"append_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"get_len_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"get_recent_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"num\": 42}",
        "{\"cmdType\": \"cdb\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"a_auth\", \"id\": \"ID\", \"token\":\"TOKEN\"}",
        "{\"cmdType\": \"g_auth\", \"id_token\": \"ID_TOKEN\"}",
        "{\"cmdType\": \"event\", \"event_id\":\"ID\", \"destination\":\"DEST\", \"origin\": \"ORIGIN\", \"data\": \"DATA\"}"];

    let cmd_types = vec![Commands::WriteToDisk, Commands::ReadFromDisk, Commands::ListDatabases,
                                      Commands::GetValue, Commands::SetValue, Commands::GetIndex,
                                      Commands::SetIndex, Commands::AppendIndex, Commands::GetLengthIndex,
                                      Commands::GetRecentIndex, Commands::CreateDatabase, Commands::AcronymAuth,
                                      Commands::GoogleAuth, Commands::Event];

    for (example, desired) in examples.iter().zip(cmd_types.iter())
    {
        let r = Command::from_string(example);
        if !r.is_ok()
        {
            panic!("Recieved an error for ${}$: {:?}", example, r.unwrap_err());
        }

        if test_output
        {
            assert_eq!(r.unwrap().cmd, *desired);
        }
    }
}

#[test]
pub fn test_command_parsing_success()
{
    test_command_parsing(false);
}

#[test]
pub fn test_command_parsing_output()
{
    test_command_parsing(true);
}

#[test]
pub fn test_command_parsing_json_failure()
{
    let examples = vec![
        "{\"cmdType\": \"wtd\", db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"rfd\", \"db_key\": \"DB_KEY\"",
        "{\"cmdType\": \"list_databases\", \"db_key\" \"DB_KEY\"}",
        "{cmdType\": \"get_val\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}"];

    for example in examples
    {
        let r = Command::from_string(example);
        assert!(r.is_err());

        if !r.is_err()
        {
            panic!("Did not recieve an error for ${}$", example);
        }

        match &r.unwrap_err()
        {
            CommandParsingError::BadJSON(_) => {},
            CommandParsingError::BadPacket(s) => panic!("Recieved BadPacket({:?}) instead for ${}$", s, example),
            CommandParsingError::ArgumentsNotPresent(s) => panic!("Recieved ArgumentsNotPresent({:?}) instead for ${}$", s, example),
        }
    }
} 

#[test]
pub fn test_command_parsing_packet_failure()
{
    let examples = vec![
        "{\"cmdType\": \"wt\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"rfd\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": [0, 1, 2, 3], \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": {\"a\": 0, \"b\": 1},\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}"];

    for example in examples
    {
        let r = Command::from_string(example);
        
        if !r.is_err()
        {
            panic!("Did not recieve an error for ${}$", example);
        }

        match &r.unwrap_err()
        {
            CommandParsingError::BadJSON(s) => panic!("Recieved BadJSON({:?}) instead for ${}$", s, example),
            CommandParsingError::BadPacket(_) => {},
            CommandParsingError::ArgumentsNotPresent(s) => panic!("Recieved ArgumentsNotPresent({:?}) instead for ${}$", s, example),
        }
    }
}