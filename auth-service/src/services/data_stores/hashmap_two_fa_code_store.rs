use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    pub codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email.clone(), (login_attempt_id, code));
        Ok(())
    }
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(&email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(value) => Ok(value.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Email;
    use secrecy::Secret;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_add_method() {
        let email = Email::parse(Secret::new("email@email.com".to_owned()))
            .expect("Failed to create email");
        let mut store = HashmapTwoFACodeStore {
            codes: HashMap::new(),
        };
        let login_attempt_id =
            LoginAttemptId::parse(Uuid::new_v4().to_string()).expect("Failed to parse uuid");
        let two_fa_code = TwoFACode::parse("123456".to_owned()).expect("Failed to parse code");

        let inserted = store
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await;
        assert!(inserted.is_ok());
        assert_eq!(store.codes.is_empty(), false);
    }

    #[tokio::test]
    async fn test_get_code() {
        let email = Email::parse(Secret::new("email@email.com".to_owned()))
            .expect("Failed to create email");
        let mut store = HashmapTwoFACodeStore {
            codes: HashMap::new(),
        };
        let login_attempt_id =
            LoginAttemptId::parse(Uuid::new_v4().to_string()).expect("Failed to parse uuid");
        let two_fa_code = TwoFACode::parse("123456".to_owned()).expect("Failed to parse code");

        let inserted = store
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await;
        assert!(inserted.is_ok());
        assert_eq!(store.codes.is_empty(), false);

        let code = store.get_code(&email).await.expect("Failed to get code");
        let (id, code) = code;
        assert_eq!(id.clone(), login_attempt_id);
        assert_eq!(code.clone(), two_fa_code);

        LoginAttemptId::parse(id.as_ref().to_string()).expect("Failed to parse login attempt id");
        TwoFACode::parse(code.as_ref().to_string()).expect("Failed to parse 2FA code");
    }

    #[tokio::test]
    async fn test_remove_method() {
        let email = Email::parse(Secret::new("email@email.com".to_owned()))
            .expect("Failed to create email");
        let mut store = HashmapTwoFACodeStore {
            codes: HashMap::new(),
        };
        let login_attempt_id = LoginAttemptId::default();
        let two_fa_code = TwoFACode::default();

        let inserted = store
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await;
        assert!(inserted.is_ok());
        assert_eq!(store.codes.is_empty(), false);

        let code = store.remove_code(&email).await;
        assert!(code.is_ok());
        assert!(store.codes.is_empty())
    }

    #[tokio::test]
    async fn test_code_not_found() {
        let email = Email::parse(Secret::new("email@email.com".to_owned()))
            .expect("Failed to create email");

        let mut store = HashmapTwoFACodeStore {
            codes: HashMap::new(),
        };

        let result = store.remove_code(&email).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TwoFACodeStoreError::LoginAttemptIdNotFound
        )
    }
}
