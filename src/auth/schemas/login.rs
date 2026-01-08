use serde::{Deserialize, Serialize};

use crate::consts::GrantType;

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct LoginRequestDto {
    pub grant_type: GrantType,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct LoginResponseDto {
    pub grant_type: GrantType,
    pub client_id: String,
    pub client_secret: String,
}

impl LoginResponseDto {
    // A public associated function named 'new' acts as the constructor
    pub fn new(login_request: LoginRequestDto) -> Self {
        Self {
            // Self refers to the struct type (LoginResponse)
            grant_type: login_request.grant_type,
            client_id: login_request.client_id,
            client_secret: login_request.client_secret,
        }
    }
}
