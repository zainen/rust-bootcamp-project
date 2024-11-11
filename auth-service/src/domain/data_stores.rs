use rand::Rng;

use super::{Email, Password, User};
use color_eyre::Report;
use thiserror::Error;

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn verify_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn verify_token_exists(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    // #[error("Token already exists")]
    // TokenAlreadyExists,
    // #[error("Token not found")]
    // TokenNotFound,
    // #[error("Invalid token")]
    // InvalidToken,
    // #[error("Unexpected error")]
    // UnexpectedError(#[source] Report),
    TokenAlreadyExists,
    TokenNotFound,
    InvalidToken,
    UnexpectedError,
}

// impl PartialEq for BannedTokenStoreError {
//     fn eq(&self, other: &Self) -> bool {
//         matches!(
//             (self, other),
//             (Self::TokenAlreadyExists, Self::TokenAlreadyExists)
//                 | (Self::TokenNotFound, Self::TokenNotFound)
//                 | (Self::InvalidToken, Self::InvalidToken)
//                 | (Self::UnexpectedError(_), Self::UnexpectedError(_))
//         )
//     }
// }

#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    // #[error("Login attept id not found")]
    // LoginAttemptIdNotFound,
    // #[error("Unexpected error")]
    // UnexpectedError(#[source] Report),
    LoginAttemptIdNotFound,
    UnexpectedError,
}

// impl PartialEq for TwoFACodeStoreError {
//     fn eq(&self, other: &Self) -> bool {
//         matches!(
//         (self, other),
//         (Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
//             | (Self::UnexpectedError(_), Self::UnexpectedError(_))
//     )
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        match uuid::Uuid::parse_str(&id) {
            Ok(val) => Ok(LoginAttemptId(val.to_string())),
            Err(_) => Err("Failed to parse uuid".to_owned()),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        let code_u32 = match code.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Err("Failed to parse code".to_owned()),
        }?;

        if (100_000..999_999).contains(&code_u32) {
            Ok(Self(code))
        } else {
            Err("Invalid 2FA code".to_owned())
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let code = rng.gen_range(100000..999999);
        Self(code.to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
