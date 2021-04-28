use super::Database;
use super::Permission;
use super::UserAuthentication;

use log::{trace, error, warn};
use chashmap::CHashMap;
use serde_json::Value;

use std::sync::Arc;

/// Database interface
#[derive(Debug, Clone)]
pub struct DatabaseInterface
{
    pub database: Arc<Database>,
    pub permissions: Arc<CHashMap<String, Permission>>
}

impl DatabaseInterface
{
    /// Generate a new database interface from a database and a set of permissions
    pub fn new(database: Database, permissions: CHashMap<String, Permission>) -> Self
    {
        trace!("Creating a new database interface for database named `{}`", &database.get_name());

        Self
        {
            database: Arc::new(database),
            permissions: Arc::new(permissions)
        }
    }

    /// Verify a user can read from a key
    fn check_read(&self, key: &str, user: &UserAuthentication) -> Result<(), String>
    {
        if let Some(permissions) = self.permissions.get(key)
        {
            if !permissions.check_user_read(user)?
            {
                let msg = format!("User not authenticated {:?}", user);
                warn!("{}", msg);
                return Err(msg);
            }
        }
        else
        {
            let msg = format!("Key `{}` does not have permissions", key);
            error!("{}", msg);
            return Err(msg);
        }

        Ok(())
    }

    /// Verify a user can write to a key
    fn check_write(&self, key: &str, user: &UserAuthentication, add_new_permission: bool) -> Result<(), String>
    {
        if let Some(permissions) = self.permissions.get(key)
        {
            if !permissions.check_user_write(user)?
            {
                let msg = format!("User not authenticated {:?}", user);
                warn!("{}", msg);
                return Err(msg);
            }
        }
        else
        {
            if !add_new_permission
            {
                let msg = format!("Key `{}` does not have permissions", key);
                error!("{}", msg);
                return Err(msg);
            }
            else
            {
                warn!("Key `{}` does not yet exist, creating default permissions for it", key);
                self.register_new_permission(key)?;
            }
        }

        Ok(())
    }

    /// Register a new permission
    fn register_new_permission(&self, name: &str) -> Result<(), String>
    {
        trace!("Registering a new permission `{}` to the interface for database `{}`", name, self.database.get_name());
        self.permissions.insert(name.to_string(), Permission::default());

        Ok(())
    }

    /// Write to a key in the database
    pub fn write_to_key(&self, key: &str, data: Value, user: &UserAuthentication) -> Result<(), String>
    {
        self.check_write(key, user, true)?;
        self.database.write(key, data)
    }

    /// Read from a key in the database
    pub fn read_from_key(&self, key: &str, user: &UserAuthentication) -> Result<Value, String>
    {
        self.check_read(key, user)?;
        self.database.read(key)
    }

    /// Write to an index in a key in the database
    pub fn write_to_key_index(&self, key: &str, index: usize, data: Value, user: &UserAuthentication) -> Result<(), String>
    {
        self.check_write(key, user, false)?;
        self.database.write_index(key, index, data)
    }

    /// Read from an index into a key in the database
    pub fn read_from_key_index(&self, key: &str, index: usize, user: &UserAuthentication) -> Result<Value, String>
    {
        self.check_read(key, user)?;
        self.database.read_index(key, index)
    }

    /// Append to an array in a key in the database
    pub fn append_to_key(&self, key: &str, data: Value, user: &UserAuthentication) -> Result<usize, String>
    {
        self.check_write(key, user, false)?;
        self.database.append(key, data)
    }

    /// Get the length of an array in a key in the database
    pub fn get_length_from_key(&self, key: &str, user: &UserAuthentication) -> Result<usize, String>
    {
        self.check_read(key, user)?;
        self.database.get_length(key)
    }

    /// Get the last n values in an array in a key in the database
    pub fn read_last_n_from_key(&self, key: &str, n: usize, user: &UserAuthentication) -> Result<Value, String>
    {
        self.check_read(key, user)?;
        self.database.get_last_n(key, n)
    }
}