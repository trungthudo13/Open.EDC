use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfoResponse {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}
