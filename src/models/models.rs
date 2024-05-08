use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    pub access_token: String,
    pub expires_in: String,
}
