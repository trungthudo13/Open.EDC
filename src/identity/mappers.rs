
impl From<LoginRequestDto> for LoginCommand {
    fn from(dto: LoginRequestDto) -> Self {
        Self {
            email: dto.email,
            password: dto.password,
        }
    }
}
