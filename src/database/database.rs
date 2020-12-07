use serde_json::Value;
use chashmap::CHashMap;
use log::{trace, error, debug};

use std::sync::Arc;

/// Database object per ACI documentation
#[derive(Debug, Clone)]
pub struct Database
{
    name: String,
    data: Arc<CHashMap<String, Value>>
}

impl Database
{
    /// Create a new empty database
    pub fn new(name: &str) -> Self
    {
        trace!("Creating an empty database named `{}`", name);

        Self
        {
            name: name.to_string(),
            data: Arc::new(CHashMap::new())
        }
    }

    /// Create a new database from a CHashMap
    pub fn create(name: &str, data: CHashMap<String, Value>) -> Self
    {
        trace!("Creating a non-empty database named `{}`", name);

        Self
        {
            name: name.to_string(),
            data: Arc::new(data)
        }
    }

    /// Write to the Database
    pub fn write(&self, key: &str, data: Value) -> Result<(), String>
    {
        trace!("Writing {} to `{}` in database {}", data, key, self.name);

        self.data.insert(key.to_string(), data);
        Ok(())
    }

    /// Read from the Database
    pub fn read(&self, key: &str) -> Result<Value, String>
    {
        trace!("Reading data from `{}` in database `{}`", key, self.name);

        if self.data.contains_key(key)
        {
            Ok(self.data.get(key).unwrap().clone())
        }
        else
        {
            let msg = format!("Key `{}` not found in database", key);
            error!("{}", msg);
            Err(msg)
        }
    }

    /// Get an entry from the database which must be an array
    fn read_key_array(&self, key: &str) -> Result<Vec<Value>, String>
    {
        if self.data.contains_key(key)
        {
            match self.data.get(key).unwrap().clone()
            {
                Value::Array(array) => Ok(array),
                default => 
                {
                    let msg = format!("The value for key `{}` is not an array ({:?})", key, default);
                    error!("{}", msg);
                    Err(msg)
                }
            }
        }
        else
        {
            let msg = format!("Key `{}` not found in database", key);
            error!("{}", msg);
            Err(msg)
        }
    }

    /// Get the index within an array stored in the hashmap
    pub fn read_index(&self, key: &str, index: usize) -> Result<Value, String>
    {
        trace!("Reading data from index `{}` in key `{}` in database {}", index, key, self.name);

        let array = self.read_key_array(key)?;

        match array.get(index)
        {
            Some(val) => Ok(val.clone()),
            None =>
            {
                let msg = format!("The array for `{}` does not contain index {}", key, index);
                error!("{}", msg);
                Err(msg)
            }
        }
    }

    /// Set the value stored at an index in an array stored in the hashmap
    pub fn write_index(&self, key: &str, index: usize, data: Value) -> Result<(), String>
    {
        trace!("Writing {} to index `{}` in key `{}` in database {}", data, index, key, self.name);

        let mut array = self.read_key_array(key)?;

        if index >= array.len()
        {
            debug!("Needing to add data to `{}` in database `{}`", key, self.name);
        }

        while index >= array.len()
        {
            array.push(Value::Null);
        }

        array[index] = data;

        self.write(key, Value::Array(array))?;
        Ok(())
    }

    /// Append to an array stored in the hashmap
    pub fn append(&self, key: &str, data: Value) -> Result<(), String>
    {
        trace!("Appending {} to `{}` in database {}", data, key, self.name);

        let mut array = self.read_key_array(key)?;

        array.push(data);

        self.write(key, Value::Array(array))?;
        Ok(())
    }

    /// Gets the length of an array stored in the hashmap
    pub fn get_length(&self, key: &str) -> Result<usize, String>
    {
        trace!("Getting length of `{}` in database {}", key, self.name);

        Ok(self.read_key_array(key)?.len())
    }

    /// Gets the last `n` items from an array stored in the hashmap, or if the length of the array is less than `n` items,
    /// return the entire array, reversed such that the order is always the highest index on the server is given the lowest
    /// index in the returned array
    pub fn get_last_n(&self, key: &str, n: usize) -> Result<Value, String>
    {
        trace!("Getting last {} items in `{}` in database {}", n, key, self.name);

        let mut array = self.read_key_array(key)?;
        let l = array.len() - n.min(array.len());

        let s = &mut array[l..];
        s.reverse();

        Ok(Value::Array(Vec::from(s)))
    }

    /// Gets the name of the database
    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }

    /// Gets the number of keys stored in the database
    pub fn get_number_of_keys(&self) -> usize
    {
        self.data.len()
    }
}