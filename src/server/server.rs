use chashmap::CHashMap;

use crate::database::DatabaseInterface;

use std::sync::Arc;

/// Server
#[derive(Debug, Clone)]
pub struct Server
{
    /// Database Interfaces
    databases: Arc<CHashMap<String, DatabaseInterface>>
}