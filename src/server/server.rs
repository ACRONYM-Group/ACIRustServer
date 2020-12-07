use chashmap::CHashMap;

use crate::database::{DatabaseInterface, UserAuthentication};

use std::sync::Arc;

/// Server
#[derive(Debug, Clone)]
pub struct Server
{
    /// Database Interfaces
    databases: Arc<CHashMap<String, DatabaseInterface>>
}