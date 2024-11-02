use super::AuthAPIError;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Password(String);

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Password {
    pub fn parse(password: String) -> Result<Password, AuthAPIError> {
        if password.len() < 8 {
            Err(AuthAPIError::InvalidCredentials)
        } else {
            Ok(Password(password))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_password_valid() {
        let given_password = "long-enough".to_owned();
        let password = Password::parse(given_password.clone());
        assert_eq!(password.unwrap().as_ref(), &given_password)
    }

    #[tokio::test]
    async fn test_invalid_password() {
        let given_password = "bad".to_owned();
        let password = Password::parse(given_password.clone());
        assert_eq!(password, Err(AuthAPIError::InvalidCredentials))
    }
}
