use serde_json::Value;
use log::{error, warn, info, debug, trace};

use super::{Database, Permission, DatabaseInterface};
use crate::args::Arguments;

use chashmap::CHashMap;

use crate::{BUILD_VERSION, COMPATIBLE_VERSIONS};

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

/// Make sure a map contains all of the necessary keys
fn ensure_keys(map: &serde_json::Map<String, Value>, keys: &[&str]) -> Result<(), String>
{
    for key in keys
    {
        if !map.contains_key(*key)
        {
            let msg = format!("Object read from database file does not contain necessary key `{}`", key);
            error!("{}", msg);
            return Err(msg);
        }
    }

    Ok(())
}

/// Read in json from a file
fn read_json(path: &str) -> Result<Value, String>
{
    let data = match std::fs::read_to_string(path)
    {
        Ok(data) => data,
        Err(e) => 
        {
            let msg = format!("Unable to read file `{}` ({})", path, e);
            error!("{}", msg);
            return Err(msg);
        }
    };

    // Parse the database from json read from the file
    let value: Value = match serde_json::from_str(&data)
    {
        Ok(v) => v,
        Err(e) =>
        {
            let msg = format!("Unable to parse json from file `{}` ({})", path, e);
            error!("{}", msg);
            return Err(msg);
        }
    };

    Ok(value)
}

/// Load a database from disk
pub fn database_from_disk(path: &std::path::PathBuf, name: &str, opt: &Arguments) -> Result<DatabaseInterface, String>
{
    info!("Loading database `{}` from {:?}", name, path);

    // Convert path to a string, and correct if it does not end with a '/'
    let mut path = if let Some(p) = path.to_str()
    {
        p.to_string()
    }
    else
    {
        let msg = format!("Unable to interpret path {:?}", path);
        error!("{}", msg);
        return Err(msg);
    };

    if !path.ends_with("/")
    {
        path += "/";
    }

    // Load database file
    let database_path = format!("{0}{1}/{1}.database", path, name);
    debug!("Loading from `{}`", database_path);

    // Ensure the database file is a map
    let map = extract_object(&read_json(&database_path)?, "Database file data")?;

    // Ensure the map contains all of the necessary fields
    ensure_keys(&map, &["dbKey", "keys", "ver"])?;

    // Check the version against the current version
    let ver = extract_string(map.get("ver").unwrap(), "Database version")?;

    if ver != BUILD_VERSION
    {
        warn!("Database `{}` is from version `{}`, while the current version is `{}`", name, ver, BUILD_VERSION);
        if (!opt.allow_all && !COMPATIBLE_VERSIONS.contains(&ver.as_str())) || opt.mismatch
        {
            if opt.mismatch
            {
                warn!("Erroring due to mismatch flag");
            }

            let msg = format!("Database version `{}` is not compatible with the current version `{}`", ver, BUILD_VERSION);
            error!("{}", msg);
            return Err(msg);
        }
    }

    // Check if the stated key is the same as the key given in the config file
    let database_key = extract_string(map.get("dbKey").unwrap(), "Database Key")?;

    if database_key != name
    {
        warn!("Database key `{}` from config file `{}` is not the same as the key expected `{}`", database_key, database_path, name);
        warn!("Database will be entered as `{}`", database_key);
    }

    // Make sure the item keys are stored in an array
    let item_keys = if let Value::Array(keys) = map.get("keys").unwrap()
    {
        keys
    }
    else
    {
        let msg = format!("Keys entry in database file is not an array");
        error!("{}", msg);
        return Err(msg);
    };

    let database_data: CHashMap<String, Value> = CHashMap::new();
    let permissions: CHashMap<String, Permission> = CHashMap::new();

    // Read each item in from its own files
    for key in item_keys
    {
        // Read in data from file
        let key = extract_string(key, &format!("Key entry `{}`", key))?;
        let item_path = &format!("{}{}/{}.item", path, name, key);
        let json = read_json(item_path)?;

        // Ensure the item data is an object
        let map = extract_object(&json, &format!("Key entry data for key `{}`", key))?;

        // Ensure the data has the proper fields
        ensure_keys(&map, &["key", "value", "permissions"])?;

        // If the keys mismatch, warn
        let found_key = extract_string(map.get("key").unwrap(), &format!("Key for item `{}`", key))?;
        if found_key != key
        {
            warn!("Item key `{}` from config file `{}` is not the same as the key expected `{}`", found_key, item_path, key);
            warn!("Item will be entered as `{}`", found_key);
        }

        // Insert the data
        database_data.insert(found_key.clone(), map.get("value").unwrap().clone());

        // Create the permissions
        permissions.insert(found_key.clone(), Permission::new(map.get("permissions").unwrap(), &found_key)?);
    }

    Ok(DatabaseInterface::new(Database::create(&database_key, database_data), permissions))
}

/// Write a database to disk
pub fn database_to_disk(path: &std::path::PathBuf, database: DatabaseInterface, _: &Arguments) -> Result<(), String>
{
    let name = database.database.get_name();

    info!("Writing database `{}` to {:?}", name, path);

    // Convert path to a string, and correct if it does not end with a '/'
    let mut path = if let Some(p) = path.to_str()
    {
        p.to_string()
    }
    else
    {
        let msg = format!("Unable to interpret path {:?}", path);
        error!("{}", msg);
        return Err(msg);
    };

    if !path.ends_with("/")
    {
        path += "/";
    }

    // Create the root path to the database
    path = format!("{}{}/", path, name);

    debug!("Using path `{}`", path);

    // Make sure the directory exists
    match std::fs::create_dir_all(&path)
    {
        Ok(()) => {},
        Err(e) => 
        {
            let msg = format!("Unable to create database directory `{}`, {}", path, e);
            error!("{}", msg);
            return Err(msg);
        }
    }

    // Produce the database JSON
    let keys = database.database.get_all_keys()?;
    let database_json = serde_json::json!({"dbKey": name, "ver": BUILD_VERSION, "keys": &keys});
    let database_file_path = format!("{}{}.database", path, name);

    info!("Writing database data to `{}`", database_file_path);

    match std::fs::write(&database_file_path, database_json.to_string())
    {
        Ok(()) => {},
        Err(e) => 
        {
            let msg = format!("Unable to write to file `{}`, {}", database_file_path, e);
            error!("{}", msg);
            return Err(msg);
        }
    }

    // Produce the files for each key
    for key in keys
    {
        trace!("Writing data for key `{}` in database `{}`", key, name);

        // Get the permissions
        let perm = match database.permissions.get(&key)
        {
            Some(v) => v,
            None => 
            {
                let msg = format!("Key `{}` in database `{}` has not permissions set", key, name);
                error!("{}", msg);
                return Err(msg);
            }
        }.clone();

        // Produce the json for the permission
        let perm_json = perm.create_json()?;

        // Produce the json for the file
        let value = database.database.read(&key)?;

        let type_str = match value
        {
            Value::Array(_) => "table",
            Value::Object(_) => "obj",
            _ => "string"
        };

        let item_json = serde_json::json!({"key": key, "value": value, "owner": "self", "permissions": perm_json, "subs": [], "type": type_str});

        let item_file_path = format!("{}{}.item", path, key);

        info!("Writing item data to `{}`", item_file_path);

        // Write json data to the file
        match std::fs::write(&item_file_path, item_json.to_string())
        {
            Ok(()) => {},
            Err(e) => 
            {
                let msg = format!("Unable to write to file `{}`, {}", item_file_path, e);
                error!("{}", msg);
                return Err(msg);
            }
        }
    }

    Ok(())
}