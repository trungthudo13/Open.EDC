use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}

impl LoginResponse {
    // A public associated function named 'new' acts as the constructor
    pub fn new(
        grant_type: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            // Self refers to the struct type (LoginResponse)
            grant_type: grant_type.into(), // Use Into<String> for flexible arguments
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    // A public associated function named 'new' acts as the constructor
    pub fn new2(login_request: LoginRequest) -> Self {
        Self {
            // Self refers to the struct type (LoginResponse)
            grant_type: login_request.grant_type,
            client_id: login_request.client_id,
            client_secret: login_request.client_secret,
        }
    }

    // A regular method to access data
    pub fn display_info(&self) {
        println!("{} is {} years old.", self.grant_type, self.client_id);
    }
}
