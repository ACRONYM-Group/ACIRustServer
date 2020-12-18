use super::Server;

use crate::database::{DatabaseInterface, Database, UserAuthentication};
use crate::commands::{Command, Commands};

use serde_json::{Value, json};
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

/// Extract a number from a json value, or throw an error
fn extract_number(val: &Value, title: &str) -> Result<usize, String>
{
    if let Value::Number(n) = val
    {
        if n.is_u64()
        {
            Ok(n.as_u64().unwrap() as usize)
        }
        else
        {
            let msg = format!("{} is not an unsigned int object", title);
            error!("{}", msg);
            return Err(msg);
        }
    }
    else
    {
        let msg = format!("{} is not a number object", title);
        error!("{}", msg);
        return Err(msg);
    }
}

/// Wrap a Result<Option<Value>, String> to include an optional unique ID
fn add_unique_id(prev: Result<Option<Value>, String>, unique_id: Option<&Value>) -> Result<Option<Value>, String>
{
    if unique_id.is_none()
    {
        prev
    }
    else if let Ok(v) = prev
    {
        if v.is_none()
        {
            Ok(v)
        }
        else
        {
            if let Value::Object(mut obj) = v.clone().unwrap()
            {
                obj.insert("unique_id".to_string(), unique_id.unwrap().clone());

                Ok(Some(Value::Object(obj)))
            }
            else
            {
                Ok(v)
            }
        }
    }
    else
    {
        prev
    }
}

/// Server Interface (to be used by individual connections)
#[derive(Debug, Clone)]
pub struct ServerInterface
{
    server: Arc<Server>,
    pub user_profile: UserAuthentication
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

        let unique_id = cmd_map.get("unique_id").clone();

        add_unique_id(match command.cmd
        {
            Commands::ReadFromDisk =>
            {
                self.is_auth("ReadFromDisk")?;

                let db_key = extract_string(cmd_map.get("db_key").unwrap(), "database key")?;

                self.server.read_database_from_disk(&db_key)?;
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": db_key})))
            },
            Commands::WriteToDisk =>
            {
                self.is_auth("WriteToDisk")?;

                let db_key = extract_string(cmd_map.get("db_key").unwrap(), "database key")?;

                self.server.write_database_to_disk(&db_key)?;
                Ok(Some(json!({"cmd": "write_to_disk", "mode": "ok", "msg": "", "db_key": db_key})))
            },
            Commands::ListDatabases =>
            {
                self.is_auth("ListDatabases")?;

                let db_key = extract_string(cmd_map.get("db_key").unwrap(), "database key")?;

                let keys = self.server.get_keys(&db_key)?;

                Ok(Some(json!({"cmd": "list_keys", "mode": "ok", "msg": "", "db_key": db_key, "val": keys})))
            },
            Commands::GetValue =>
            {
                self.is_auth("GetValue")?;

                let db_key = &extract_string(cmd_map.get("db_key").unwrap(), "database key")?;
                let key = &extract_string(cmd_map.get("key").unwrap(), "item key")?;

                let data = match self.server.databases.get(db_key)
                {
                    Some(v) => {v.read_from_key(&key, &self.user_profile)?},
                    None => 
                    {
                        let msg = format!("No database with key `{}` initialized", db_key);
                        error!("{}", msg);
                        return Err(msg);
                    }
                };

                Ok(Some(json!({"cmd": "get_value", "mode": "ok", "msg": "", "key": key, "db_key": db_key, "val": data})))
            },
            Commands::SetValue =>
            {
                self.is_auth("SetValue")?;

                let db_key = &extract_string(cmd_map.get("db_key").unwrap(), "database key")?;
                let key = &extract_string(cmd_map.get("key").unwrap(), "item key")?;

                let data = cmd_map.get("val").unwrap();

                match self.server.databases.get(db_key)
                {
                    Some(v) => {v.write_to_key(&key, data.clone(), &self.user_profile)?},
                    None => 
                    {
                        let msg = format!("No database with key `{}` initialized", db_key);
                        error!("{}", msg);
                        return Err(msg);
                    }
                };

                Ok(Some(json!({"cmd": "set_value", "mode": "ok", "msg": "", "key": key, "db_key": db_key,})))
            },
            Commands::GetIndex =>
            {
                self.is_auth("GetIndex")?;

                let db_key = &extract_string(cmd_map.get("db_key").unwrap(), "database key")?;
                let key = &extract_string(cmd_map.get("key").unwrap(), "item key")?;
                let index = extract_number(cmd_map.get("index").unwrap(), "index")?;

                let data = match self.server.databases.get(db_key)
                {
                    Some(v) => {v.read_from_key_index(&key, index, &self.user_profile)?},
                    None => 
                    {
                        let msg = format!("No database with key `{}` initialized", db_key);
                        error!("{}", msg);
                        return Err(msg);
                    }
                };

                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "key": key, "db_key": db_key, "index": index, "val": data})))
            },
            Commands::SetIndex =>
            {
                self.is_auth("SetIndex")?;

                let db_key = &extract_string(cmd_map.get("db_key").unwrap(), "database key")?;
                let key = &extract_string(cmd_map.get("key").unwrap(), "item key")?;
                let index = extract_number(cmd_map.get("index").unwrap(), "index")?;

                let data = cmd_map.get("val").unwrap();

                match self.server.databases.get(db_key)
                {
                    Some(v) => {v.write_to_key_index(&key, index, data.clone(), &self.user_profile)?},
                    None => 
                    {
                        let msg = format!("No database with key `{}` initialized", db_key);
                        error!("{}", msg);
                        return Err(msg);
                    }
                };

                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "key": key, "db_key": db_key, "index": index})))
            },
            Commands::AppendIndex =>
            {
                self.is_auth("AppendIndex")?;

                let db_key = &extract_string(cmd_map.get("db_key").unwrap(), "database key")?;
                let key = &extract_string(cmd_map.get("key").unwrap(), "item key")?;

                let data = cmd_map.get("val").unwrap();

                let index = match self.server.databases.get(db_key)
                {
                    Some(v) => {v.append_to_key(&key, data.clone(), &self.user_profile)?},
                    None => 
                    {
                        let msg = format!("No database with key `{}` initialized", db_key);
                        error!("{}", msg);
                        return Err(msg);
                    }
                };

                Ok(Some(json!({"cmd": "append_list", "mode": "ok", "msg": "", "key": key, "db_key": db_key, "next": index})))
            },
            Commands::GetRecentIndex =>
            {
                self.is_auth("GetRecentIndex")?;

                let db_key = &extract_string(cmd_map.get("db_key").unwrap(), "database key")?;
                let key = &extract_string(cmd_map.get("key").unwrap(), "item key")?;
                let index = extract_number(cmd_map.get("num").unwrap(), "read window")?;

                let data = match self.server.databases.get(db_key)
                {
                    Some(v) => {v.read_last_n_from_key(&key, index, &self.user_profile)?},
                    None => 
                    {
                        let msg = format!("No database with key `{}` initialized", db_key);
                        error!("{}", msg);
                        return Err(msg);
                    }
                };

                Ok(Some(json!({"cmd": "get_recent", "mode": "ok", "msg": "", "key": key, "db_key": db_key, "val": data})))
            },
            Commands::GetLengthIndex => 
            {
                self.is_auth("GetLengthIndex")?;

                let db_key = &extract_string(cmd_map.get("db_key").unwrap(), "database key")?;
                let key = &extract_string(cmd_map.get("key").unwrap(), "item key")?;

                let length = match self.server.databases.get(db_key)
                {
                    Some(v) => {v.get_length_from_key(&key, &self.user_profile)?},
                    None => 
                    {
                        let msg = format!("No database with key `{}` initialized", db_key);
                        error!("{}", msg);
                        return Err(msg);
                    }
                };

                Ok(Some(json!({"cmd": "get_list_length", "mode": "ok", "msg": "", "key": key, "db_key": db_key, "length": length})))
            },
            Commands::CreateDatabase =>
            {
                self.is_auth("CreateDatabase")?;

                let name = extract_string(cmd_map.get("db_key").unwrap(), "database key")?;

                self.server.databases.insert(name.clone(), DatabaseInterface::new(Database::new(&name), chashmap::CHashMap::new()));
                Ok(Some(json!({"cmd": "create_database", "mode": "ok", "msg": "", "db_key": name})))
            },
            Commands::AcronymAuth =>
            {
                let id = extract_string(cmd_map.get("id").unwrap(), "user id")?;
                let token = extract_string(cmd_map.get("token").unwrap(), "user token")?;

                let (result, msg) = self.server.check_a_auth(&id, &token)?;

                if result
                {
                    self.user_profile.is_authed = true;
                    self.user_profile.domain = "a_auth".to_string();
                    self.user_profile.name = id;
                }

                Ok(Some(json!({"cmd": "a_auth", "mode": "ok", "msg": msg})))
            },
            Commands::GoogleAuth =>
            {
                let id = extract_string(cmd_map.get("id_token").unwrap(), "user id")?;

                let name = super::authentication::google_authenticate(&id)?;

                match name
                {
                    Some(value) => 
                    {
                        self.user_profile.is_authed = true;
                        self.user_profile.domain = "g_auth".to_string();
                        self.user_profile.name = value;

                        Ok(Some(json!({"cmd": "a_auth", "mode": "ok", "msg": "success"})))
                    },
                    None =>
                    {
                        Ok(Some(json!({"cmd": "a_auth", "mode": "ok", "msg": "error"})))
                    }
                }
            },
            Commands::Event =>
            {
                let msg = format!("Event command should never make it to the server interface");
                error!("{}", msg);
                Err(msg)
            }
        }, unique_id)
    }
}