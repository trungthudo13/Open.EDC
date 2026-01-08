pub struct PgUserRepo {
    // pool: PgPool, ...
}

impl UserRepo for PgUserRepo {
    fn get_name(&self, user_id: i64) -> Option<String> {
        // query db...
        Some(format!("user-{user_id}"))
    }
}
