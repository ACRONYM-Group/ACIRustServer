use super::Server;

use crate::database::UserAuthentication;

use std::sync::Arc;

/// Server Interface (to be used by individual connections)
#[derive(Debug, Clone)]
pub struct ServerInterface
{
    server: Arc<Server>,
    user_profile: UserAuthentication
}

impl ServerInterface
{
    /// Create a fresh connection to the server
    pub fn new(server: &Arc<Server>) -> Self
    {
        Self
        {
            server: server.clone(),
            user_profile: UserAuthentication::new()
        }
    }
}