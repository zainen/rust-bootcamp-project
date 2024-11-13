use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Email(Secret<String>);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl Eq for Email {}

impl Email {
    pub fn parse(email: Secret<String>) -> Result<Email> {
        if validate_email(email.expose_secret()) {
            Ok(Self(email))
        } else {
            Err(eyre!("{} is not a valid email.", email.expose_secret()))
        }
    }
}

fn validate_email(s: &str) -> bool {
    s.contains("@")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_valid() {
        let given_email = "something@something.com".to_owned();
        let email = Email::parse(Secret::new(given_email.clone()));
        let result = email.unwrap();
        assert_eq!(result.as_ref(), &given_email);
    }

    #[tokio::test]
    async fn test_invalid_email() {
        let given_email = "something.com".to_owned();
        let email = Email::parse(Secret::new(given_email.clone()));
        assert!(email.is_err());
    }
}
