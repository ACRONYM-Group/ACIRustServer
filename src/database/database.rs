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
        unimplemented!()
    }

    /// Read from the Database
    pub fn read(&self, key: &str) -> Result<Value, String>
    {
        unimplemented!()
    }

    /// Get the index within an array stored in the hashmap
    pub fn read_index(&self, key: &str, index: usize) -> Result<Value, String>
    {
        unimplemented!()
    }

    /// Set the value stored at an index in an array stored in the hashmap
    pub fn write_index(&self, key: &str, index: usize, data: Value) -> Result<(), String>
    {
        unimplemented!()
    }

    /// Append to an array stored in the hashmap
    pub fn append(&self, key: &str, data: Value) -> Result<(), String>
    {
        unimplemented!()
    }

    /// Gets the length of an array stored in the hashmap
    pub fn get_length(&self, key: &str) -> Result<usize, String>
    {
        unimplemented!()
    }

    /// Gets the last `n` items from an array stored in the hashmap, or if the length of the array is less than `n` items,
    /// return the entire array, reversed such that the order is always the highest index on the server is given the lowest
    /// index in the returned array
    pub fn get_last_n(&self, key: &str, n: usize) -> Result<Value, String>
    {
        unimplemented!()
    }

    /// Gets the name of the database
    pub fn get_name(&self) -> String
    {
        unimplemented!()
    }

    /// Gets the number of keys stored in the database
    pub fn get_number_of_keys(&self) -> usize
    {
        unimplemented!()
    }
}