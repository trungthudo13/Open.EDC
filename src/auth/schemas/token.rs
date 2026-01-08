use serde::{Deserialize, Serialize};

use crate::consts::GrantType;

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct TokenRequestDto {
    pub grant_type: GrantType,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct TokenResponseDto {
    pub grant_type: GrantType,
    pub client_id: String,
    pub client_secret: String,
}

impl TokenRequestDto {
    // A public associated function named 'new' acts as the constructor
    pub fn new(token_request: TokenRequestDto) -> Self {
        Self {
            // Self refers to the struct type (TokenRequestDto)
            grant_type: token_request.grant_type,
            client_id: token_request.client_id,
            client_secret: token_request.client_secret,
        }
    }
}
