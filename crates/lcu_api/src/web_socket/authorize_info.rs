#[derive(Debug)]
pub struct AuthorizeInfo {
    pub port: String,
    pub auth_token: String,
}

impl AuthorizeInfo {
    pub fn new (port: String, auth_token: String) -> Self {
        Self {
            port,
            auth_token,
        }
    }
}