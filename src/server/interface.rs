use super::Server;

use crate::database::UserAuthentication;
use crate::commands::{Command, Commands};

use serde_json::Value;
use log::{trace, error};

use std::sync::Arc;

/// Extract a string from a json value, or throw an error
fn extract_string(val: &Value, title: &str) -> Result<String, String>
{
    if let Value::String(s) = val
    {
        Ok(s.clone())
    }
    else
    {
        let msg = format!("{} is not a string object", title);
        error!("{}", msg);
        return Err(msg);
    }
}

/// Server Interface (to be used by individual connections)
#[derive(Debug, Clone)]
pub struct ServerInterface
{
    server: Arc<Server>,
    user_profile: UserAuthentication
}

impl ServerInterface
{
    /// Create a fresh connection to the server
    pub fn new(server: &Arc<Server>) -> Self
    {
        Self
        {
            server: server.clone(),
            user_profile: UserAuthentication::new()
        }
    }

    /// Ensure the user is authenticated
    pub fn is_auth(&self, operation: &str) -> Result<(), String>
    {
        if !self.user_profile.is_authed
        {
            let msg = format!("Cannot perform operation {}, user is not yet authenticated", operation);
            error!("{}", msg);
            return Err(msg);
        }

        Ok(())
    }

    /// Fake auth (for use for testing)
    pub fn fake_auth(&mut self)
    {
        self.user_profile.is_authed = true;
    }

    /// Execute a command on the database
    pub fn execute_command(&mut self, command: Command) -> Result<Option<Value>, String>
    {
        trace!("Executing command `{:?}` as {:?}", command.cmd, self.user_profile);

        let cmd_map = if let Value::Object(map) = command.data
        {
            map
        }
        else
        {
            let msg = format!("Command data is not an object");
            error!("{}", msg);
            return Err(msg);
        };

        match command.cmd
        {
            Commands::ReadFromDisk =>
            {
                self.is_auth("ReadFromDisk")?;

                self.server.read_database_from_disk(&extract_string(cmd_map.get("db_key").unwrap(), "database key")?)?;
                Ok(None)
            },
            Commands::WriteToDisk =>
            {
                self.is_auth("WriteToDisk")?;

                self.server.write_database_to_disk(&extract_string(cmd_map.get("db_key").unwrap(), "database key")?)?;
                Ok(None)
            },
            default => 
            {
                let msg = format!("Command `{:?}` not yet implemented", default);
                error!("{}", msg);
                Err(msg)
            }
        }
    }
}