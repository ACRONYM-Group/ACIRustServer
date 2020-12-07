use log::{error, trace};
use serde_json::{Value, json};

/// Permission gating a value
#[derive(Debug, Clone)]
pub struct Permission
{
    pub read_a_users: Vec<String>,
    pub read_g_users: Vec<String>,

    pub write_a_users: Vec<String>,
    pub write_g_users: Vec<String>
}

impl Permission
{
    /// Generate permissions from a json object
    pub fn new(object: &Value, name: &str) -> Result<Self, String>
    {
        trace!("Creating premissions for {}", name);

        let map = if let Value::Object(obj) = &object
        {
            obj
        }
        else
        {
            let msg = format!("Value passed to make permission is not an object {}", object);
            error!("{}", msg);
            return Err(msg);
        };

        if !map.contains_key("read")
        {
            let msg = format!("No read permissions given, got {}", object);
            error!("{}", msg);
            return Err(msg);
        }

        if !map.contains_key("write")
        {
            let msg = format!("No write permissions given, got {}", object);
            error!("{}", msg);
            return Err(msg);
        }

        let mut read_a_users: Vec<String> = vec![];
        let mut read_g_users: Vec<String> = vec![];

        let mut write_a_users: Vec<String> = vec![];
        let mut write_g_users: Vec<String> = vec![];

        if let Value::Array(read_perms) = map.get("read").unwrap()
        {
            for perm in read_perms
            {
                if let Value::Array(perm) = perm
                {
                    if perm.len() != 2
                    {
                        let msg = format!("Permission is not an array of two values, got {:?}", perm);
                        error!("{}", msg);
                        return Err(msg);
                    }

                    if let Value::String(name) = &perm[1]
                    {
                        if perm[0] == json!("a_user")
                        {
                            read_a_users.push(name.clone());
                        }
                        else if perm[0] == json!("g_user")
                        {
                            read_g_users.push(name.clone());
                        }
                        else
                        {
                            let msg = format!("Unknown permission domain {:?}", &perm[0]);
                            error!("{}", msg);
                            return Err(msg);
                        }
                    }
                    else
                    {
                        let msg = format!("Permission entity is not a string, got {:?}", &perm[1]);
                        error!("{}", msg);
                        return Err(msg);
                    }
                }
                else
                {
                    let msg = format!("Read permission is not an array, got {}", perm);
                    error!("{}", msg);
                    return Err(msg);
                }
            }
        }
        else
        {
            let msg = format!("Read permissions are not in an array, got {}", object);
            error!("{}", msg);
            return Err(msg);
        }

        if let Value::Array(write_perms) = map.get("write").unwrap()
        {
            for perm in write_perms
            {
                if let Value::Array(perm) = perm
                {
                    if perm.len() != 2
                    {
                        let msg = format!("Permission is not an array of two values, got {:?}", perm);
                        error!("{}", msg);
                        return Err(msg);
                    }

                    if let Value::String(name) = &perm[1]
                    {
                        if perm[0] == json!("a_user")
                        {
                            write_a_users.push(name.clone());
                        }
                        else if perm[0] == json!("g_user")
                        {
                            write_g_users.push(name.clone());
                        }
                        else
                        {
                            let msg = format!("Unknown permission domain {:?}", &perm[0]);
                            error!("{}", msg);
                            return Err(msg);
                        }
                    }
                    else
                    {
                        let msg = format!("Permission entity is not a string, got {:?}", &perm[1]);
                        error!("{}", msg);
                        return Err(msg);
                    }
                }
                else
                {
                    let msg = format!("Write permission is not an array, got {}", perm);
                    error!("{}", msg);
                    return Err(msg);
                }
            }
        }
        else
        {
            let msg = format!("Write permissions are not in an array, got {}", object);
            error!("{}", msg);
            return Err(msg);
        }

        Ok(
            Self
            {
                read_a_users, read_g_users,
                write_a_users, write_g_users
            }
        )
    }

    /// Check if a user is allowed to read from the gated item
    pub fn check_read(&self, is_authed: bool, user: &String, domain: &String) -> Result<bool, String>
    {
        trace!("Checking if {} user `{}`:`{}` can read", if is_authed {"Authed"} else {"Not authed"}, domain, user);

        if self.read_a_users.contains(&"any".to_string())
        {
            return Ok(true)
        }

        if !is_authed
        {
            return Ok(false)
        }

        if domain == "a_auth"
        {
            return Ok(self.read_a_users.contains(&"authed".to_string()) || self.read_a_users.contains(user)) 
        }
        else if domain == "g_auth"
        {
            return Ok(self.read_g_users.contains(&"authed".to_string()) || self.read_g_users.contains(user)) 
        }
        else
        {
            let msg = format!("Unknown permission domain given to check_read {}", domain);
            error!("{}", msg);
            return Err(msg);
        }
    }

    /// Check if a user is allowed to write to the gated item
    pub fn check_write(&self, is_authed: bool, user: &String, domain: &String) -> Result<bool, String>
    {
        trace!("Checking if {} user `{}`:`{}` can write", if is_authed {"Authed"} else {"Not authed"}, domain, user);

        if self.write_a_users.contains(&"any".to_string())
        {
            return Ok(true)
        }

        if !is_authed
        {
            return Ok(false)
        }

        if domain == "a_auth"
        {
            return Ok(self.write_a_users.contains(&"authed".to_string()) || self.write_a_users.contains(user)) 
        }
        else if domain == "g_auth"
        {
            return Ok(self.write_g_users.contains(&"authed".to_string()) || self.write_g_users.contains(user)) 
        }
        else
        {
            let msg = format!("Unknown permission domain given to check_write {}", domain);
            error!("{}", msg);
            return Err(msg);
        }
    }

    /// Wrapper around check_read for UserAuthentication
    pub fn check_user_read(&self, user: &UserAuthentication) -> Result<bool, String>
    {
        self.check_read(user.is_authed, &user.name, &user.domain)
    }

    /// Wrapper around check_write for UserAuthentication
    pub fn check_user_write(&self, user: &UserAuthentication) -> Result<bool, String>
    {
        self.check_write(user.is_authed, &user.name, &user.domain)
    }
}

impl std::default::Default for Permission
{
    fn default() -> Permission
    {
        Permission
        {
            read_a_users: vec!["any".to_string()],
            read_g_users: vec!["any".to_string()],
            write_a_users: vec!["any".to_string()],
            write_g_users: vec!["any".to_string()]
        }
    }
}

/// User authentication state
#[derive(Debug, Clone)]
pub struct UserAuthentication
{
    pub is_authed: bool,
    pub name: String,
    pub domain: String
}

impl UserAuthentication
{
    /// Create a completely fresh connection user
    pub fn new() -> Self
    {
        Self
        {
            is_authed: false,
            name: String::new(),
            domain: String::new()
        }
    }
}