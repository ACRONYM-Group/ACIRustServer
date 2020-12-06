use serde_json::Value;
use std::sync::Arc;
use chashmap::CHashMap;


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
        Self
        {
            name: name.to_string(),
            data: Arc::new(CHashMap::new())
        }
    }

    /// Write to the Database
    pub fn write(&self, key: &str, data: Value) -> Result<(), String>
    {
        self.data.insert(key.to_string(), data);
        Ok(())
    }

    /// Read from the Database
    pub fn read(&self, key: &str) -> Result<Value, String>
    {
        if self.data.contains_key(key)
        {
            Ok(self.data.get(key).unwrap().clone())
        }
        else
        {
            Err(format!("Key `{}` not found in database", key))
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
                default => Err(format!("The value for key `{}` is not an array ({:?})", key, default))
            }
        }
        else
        {
            Err(format!("Key `{}` not found in database", key))
        }
    }

    /// Get the index within an array stored in the hashmap
    pub fn read_index(&self, key: &str, index: usize) -> Result<Value, String>
    {
        let array = self.read_key_array(key)?;

        match array.get(index)
        {
            Some(val) => Ok(val.clone()),
            None => Err(format!("The array for `{}` does not contain index {}", key, index))
        }
    }

    /// Set the value stored at an index in an array stored in the hashmap
    pub fn write_index(&self, key: &str, index: usize, data: Value) -> Result<(), String>
    {
        let mut array = self.read_key_array(key)?;

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
        let mut array = self.read_key_array(key)?;

        array.push(data);

        self.write(key, Value::Array(array))?;
        Ok(())
    }

    /// Gets the length of an array stored in the hashmap
    pub fn get_length(&self, key: &str) -> Result<usize, String>
    {
        Ok(self.read_key_array(key)?.len())
    }

    /// Gets the last `n` items from an array stored in the hashmap, or if the length of the array is less than `n` items,
    /// return the entire array, reversed such that the order is always the highest index on the server is given the lowest
    /// index in the returned array
    pub fn get_last_n(&self, key: &str, n: usize) -> Result<Value, String>
    {
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