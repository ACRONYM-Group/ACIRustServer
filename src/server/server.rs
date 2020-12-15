use chashmap::CHashMap;

use crate::database::{DatabaseInterface, database_from_disk, database_to_disk, UserAuthentication};
use crate::args::Arguments;

use log::error;

use serde_json::Value;

use std::sync::Arc;

/// Extract an object from a json value, or throw an error
fn extract_object(val: &Value, title: &str) -> Result<serde_json::Map<String, Value>, String>
{
    if let Value::Object(m) = val
    {
        Ok(m.clone())
    }
    else
    {
        let msg = format!("{} is not an object", title);
        error!("{}", msg);
        return Err(msg);
    }
}

/// Server
#[derive(Debug)]
pub struct Server
{
    /// Database Interfaces
    pub databases: Arc<CHashMap<String, DatabaseInterface>>,

    /// Options
    opt: Arguments,

    /// Config Database
    config_database: Arc<DatabaseInterface>,

    /// Config Admin
    config_admin: UserAuthentication
}

impl Server
{
    /// Create a new (empty) Server
    pub fn new(opt: &Arguments) -> Result<Self, String>
    {
        let config = database_from_disk(&opt.config_path.clone(), "config", opt)?;

        let mut admin = UserAuthentication::new();
        admin.is_authed = true;

        Ok(Self
        {
            databases: Arc::new(CHashMap::new()),
            opt: opt.clone(),
            config_database: Arc::new(config),
            config_admin: admin
        })
    }

    /// Get the ip address of the Server from the config database
    pub fn config_get_ip(&self) -> Result<String, String>
    {
        if let Ok(val) = self.config_database.read_from_key("ip", &self.config_admin)
        {
            if let Value::String(ip) = val
            {
                Ok(ip)
            }
            else
            {
                let msg = "IP address field in the config database is not a string".to_string();
                log::error!("{}", msg);
                Err(msg)
            }
        }
        else
        {
            let msg = "No IP address given in the config database".to_string();
            log::error!("{}", msg);
            Err(msg)
        }
    }

    /// Get the port of the Server from the config database
    pub fn config_get_port(&self) -> Result<usize, String>
    {
        if let Ok(val) = self.config_database.read_from_key("port", &self.config_admin)
        {
            if let Value::Number(ip) = val
            {
                if ip.is_u64()
                {
                    Ok(ip.as_u64().unwrap() as usize)
                }
                else
                {
                    let msg = "Port field in the config database is not a u64".to_string();
                    log::error!("{}", msg);
                    Err(msg)
                }
            }
            else
            {
                let msg = "Port field in the config database is not an integer".to_string();
                log::error!("{}", msg);
                Err(msg)
            }
        }
        else
        {
            let msg = "No port given in the config database".to_string();
            log::error!("{}", msg);
            Err(msg)
        }
    }

    /// Read a database from disk
    pub fn read_database_from_disk(&self, name: &str) -> Result<(), String>
    {
        self.databases.insert(name.to_string(), database_from_disk(&self.opt.path.clone(), name, &self.opt)?);

        Ok(())
    }

    /// Write a database to disk
    pub fn write_database_to_disk(&self, name: &str) -> Result<(), String>
    {
        if !self.databases.contains_key(name)
        {
            let msg = format!("No database with key `{}` initialized", name);
            error!("{}", msg);
            return Err(msg);
        }

        database_to_disk(&self.opt.path.clone(), self.databases.get(name).unwrap().clone(), &self.opt)?;

        Ok(())
    }

    /// Get the array of keys in the given database
    pub fn get_keys(&self, name: &str) -> Result<Vec<String>, String>
    {
        if !self.databases.contains_key(name)
        {
            let msg = format!("No database with key `{}` initialized", name);
            error!("{}", msg);
            return Err(msg);
        }

        Ok(self.databases.get(name).unwrap().database.get_all_keys()?)
    }

    /// Check acronym authentication
    pub fn check_a_auth(&self, id: &str, token: &str) -> Result<(bool, String), String>
    {
        let user_data = self.config_database.read_from_key("a_users", &self.config_admin)?;
        let user_map = extract_object(&user_data, "General user data")?;

        if !user_map.contains_key(id)
        {
            let msg = format!("Failed, a_user not found");
            error!("{}", msg);
            return Ok((false, msg));
        }

        let id_map = extract_object(user_map.get(id).unwrap(), "Specific user data")?;

        if !id_map.contains_key("tokens")
        {
            let msg = format!("Specific user data does not contain `tokens` key");
            error!("{}", msg);
            return Err(msg);
        }

        let allowable_tokens = if let Value::Array(tokens) = id_map.get("tokens").unwrap()
        {
            tokens
        }
        else
        {
            let msg = format!("Tokens for specific user data is not an array");
            error!("{}", msg);
            return Err(msg);
        };

        if !allowable_tokens.contains(&Value::String(token.to_string()))
        {
            let msg = format!("Failed, token incorrect");
            error!("{}", msg);
            return Ok((false, msg));
        }

        Ok((true, "success".to_string()))
    }
}