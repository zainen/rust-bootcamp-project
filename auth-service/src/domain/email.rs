use super::AuthAPIError;

#[derive(Debug, PartialEq, Default, Eq, Hash, Clone)]
pub struct Email(String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn parse(email: String) -> Result<Email, AuthAPIError> {
        if email.contains("@") {
            Ok(Email (email))
        } else {
            Err(AuthAPIError::InvalidCredentials)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_valid() {
        let given_email = "something@something.com".to_owned();
        let email = Email::parse(given_email.clone());
        let result = email.unwrap();
        assert_eq!(result.as_ref(), &given_email);
    }

    #[tokio::test]
    async fn test_invalid_email() {
        let given_email = "something.com".to_owned();
        let email = Email::parse(given_email.clone());
        assert_eq!(email, Err(AuthAPIError::InvalidCredentials));
    }
}
