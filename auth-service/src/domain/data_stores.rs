use super::User;

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    fn get_user(&self, email: &String) -> Result<&User, UserStoreError>;
    fn verify_user(&self, email: &String, password: &String) -> Result<(), UserStoreError>;
}



#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
