// src/consts.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    Password,
    RefreshToken,
}

impl Default for GrantType {
    fn default() -> Self {
        GrantType::Password
    }
}
