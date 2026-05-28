#[derive(Debug, Clone, Copy)]
pub enum AddUserResult {
    Added,
    NameAlreadyExists,
    ChatIdAlreadyExists,
}

#[derive(Debug, Clone, Copy)]
pub enum RemoveUserResult {
    Removed,
    NotFound,
}
