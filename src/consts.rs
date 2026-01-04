// src/consts.rs

pub enum GrantType {
    Password,
    RefreshToken,
}

static LOGIN_METHOD: &str = "login_method";