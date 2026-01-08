struct UserRow {
    id: i32,
    username: String,
    email: String,
    is_active: bool,
}

impl UserRow {
    fn new(id: i32, username: String, email: String, is_active: bool) -> Self {
        UserRow {
            id,
            username,
            email,
            is_active,
        }
    }

    fn activate(&mut self) {
        self.is_active = true;
    }

    fn deactivate(&mut self) {
        self.is_active = false;
    }
}
