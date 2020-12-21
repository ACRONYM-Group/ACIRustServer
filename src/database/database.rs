use serde_json::Value;
use chashmap::CHashMap;
use log::{trace, error, debug};

use std::sync::Arc;

use std::ops::DerefMut;

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

    /// Verify an entry is an array
    fn verify_key_array(&self, key: &str) -> Result<(), String>
    {
        if self.data.contains_key(key)
        {
            match &*self.data.get(key).unwrap()
            {
                Value::Array(_) => Ok(()),
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

        self.verify_key_array(key)?;

        if let Value::Array(array) = &*self.data.get(key).unwrap()
        {
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
        else
        {
            let msg = format!("The value for key `{}` is not an array", key);
            error!("{}", msg);
            Err(msg)
        }
    }

    /// Set the value stored at an index in an array stored in the hashmap
    pub fn write_index(&self, key: &str, index: usize, data: Value) -> Result<(), String>
    {
        trace!("Writing {} to index `{}` in key `{}` in database {}", data, index, key, self.name);

        self.verify_key_array(key)?;

        if let Value::Array(array) = self.data.get_mut(key).unwrap().deref_mut()
        {
            if index >= array.len()
            {
                debug!("Needing to add data to `{}` in database `{}`", key, self.name);
            }

            while index >= array.len()
            {
                array.push(Value::Null);
            }

            array[index] = data;
            Ok(())
        }
        else
        {
            let msg = format!("The value for key `{}` is not an array", key);
            error!("{}", msg);
            Err(msg)
        }
    }

    /// Append to an array stored in the hashmap
    pub fn append(&self, key: &str, data: Value) -> Result<usize, String>
    {
        trace!("Appending {} to `{}` in database {}", data, key, self.name);

        self.verify_key_array(key)?;

        if let Value::Array(array) = self.data.get_mut(key).unwrap().deref_mut()
        {
            let l = array.len();
            array.push(data);

            Ok(l)
        }
        else
        {
            let msg = format!("The value for key `{}` is not an array", key);
            error!("{}", msg);
            Err(msg)
        }
    }

    /// Gets the length of an array stored in the hashmap
    pub fn get_length(&self, key: &str) -> Result<usize, String>
    {
        trace!("Getting length of `{}` in database {}", key, self.name);

        self.verify_key_array(key)?;

        if let Value::Array(array) = &*self.data.get(key).unwrap()
        {
            Ok(array.len())
        }
        else
        {
            let msg = format!("The value for key `{}` is not an array", key);
            error!("{}", msg);
            Err(msg)
        }
    }

    /// Gets the last `n` items from an array stored in the hashmap, or if the length of the array is less than `n` items,
    /// return the entire array
    pub fn get_last_n(&self, key: &str, n: usize) -> Result<Value, String>
    {
        trace!("Getting last {} items in `{}` in database {}", n, key, self.name);

        self.verify_key_array(key)?;

        if let Value::Array(array) = &*self.data.get(key).unwrap()
        {
            let l = array.len() - n.min(array.len());

            let s = &array[l..];

            Ok(Value::Array(Vec::from(s)))
        }
        else
        {
            let msg = format!("The value for key `{}` is not an array", key);
            error!("{}", msg);
            Err(msg)
        }
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

    /// Gets all of the keys in the database
    pub fn get_all_keys(&self) -> Result<Vec<String>, String>
    {
        let mut keys = vec![];
        for (k, _) in (*self.data).clone().into_iter()
        {
            keys.push(k.clone());
        }

        keys.sort();

        Ok(keys)
    }
}