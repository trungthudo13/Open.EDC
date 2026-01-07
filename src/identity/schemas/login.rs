use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct LoginRequestDto {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct LoginResponseDto {
    pub grant_type: String,
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

    // A regular method to access data
    // pub fn display_info(&self) {
    //     println!("{} is {} years old.", self.grant_type, self.client_id);
    // }
}
