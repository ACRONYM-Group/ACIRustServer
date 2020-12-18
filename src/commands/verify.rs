use super::{Command, Commands, CommandParsingError};

use serde_json::Value;

/// Verify the integrity of the command
pub fn verify_command(command: &Command) -> Result<(), CommandParsingError>
{
    let keys = match command.cmd
    {
        Commands::AcronymAuth => {["id", "token", "token", "token"]},
        Commands::AppendIndex => {["db_key", "key", "val", "val"]},
        Commands::CreateDatabase => {["db_key", "db_key", "db_key", "db_key"]},
        Commands::Event => {["event_id", "destination", "origin", "data"]},
        Commands::GetIndex => {["db_key", "key", "index", "index"]},
        Commands::GetLengthIndex => {["db_key", "key", "key", "key"]},
        Commands::GetRecentIndex => {["db_key", "key", "num", "num"]},
        Commands::GetValue => {["db_key", "key", "key", "key"]},
        Commands::GoogleAuth => {["id_token", "id_token", "id_token", "id_token"]},
        Commands::ListDatabases => {["db_key", "db_key", "db_key", "db_key"]},
        Commands::ReadFromDisk => {["db_key", "db_key", "db_key", "db_key"]},
        Commands::SetIndex => {["db_key", "key", "val", "index"]},
        Commands::SetValue => {["db_key", "key", "val", "val"]},
        Commands::WriteToDisk => {["db_key", "db_key", "db_key", "db_key"]}
    };

    if let Value::Object(object) = &command.data
    {
        for key in &keys
        {
            if !object.contains_key(&key.to_string())
            {
                let msg = format!("Command {:?} requires keys {:?}, did not find key {:?}", command.cmd, keys, key);
                log::error!("{}", msg);
                return Err(CommandParsingError::ArgumentsNotPresent(msg));
            }
        }
    }

    Ok(())
}