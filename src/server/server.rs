use chashmap::CHashMap;

use crate::database::{DatabaseInterface, database_from_disk, database_to_disk};
use crate::args::Arguments;

use log::error;

use std::sync::Arc;

/// Server
#[derive(Debug)]
pub struct Server
{
    /// Database Interfaces
    databases: Arc<CHashMap<String, DatabaseInterface>>,

    /// Options
    opt: Arguments,

    /// Database keys
    database_keys: Vec<String>
}

impl Server
{
    /// Create a new (empty) Server
    pub fn new(opt: &Arguments) -> Self
    {
        Self
        {
            databases: Arc::new(CHashMap::new()),
            opt: opt.clone(),
            database_keys: vec![]
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

    /// Get a list of all of the databases
    pub fn get_list_of_databases(&self) -> Result<Vec<String>, String>
    {
        let mut keys = vec![];
        for (k, _) in (*self.databases).clone().into_iter()
        {
            keys.push(k.clone());
        }

        Ok(keys)
    }
}