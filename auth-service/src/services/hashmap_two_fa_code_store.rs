use std::collections::HashMap;

use crate::domain::{
  data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
  email::{self, Email}
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
  pub codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
  async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode) -> Result<(), TwoFACodeStoreError> {
    if self.get_code(&email).await.is_ok() {
      return Err(TwoFACodeStoreError::CodeAlreadyExists)
    }
    self.codes.insert(email.clone(), (login_attempt_id, code));

    if self.get_code(&email).await.is_err() {
      return Err(TwoFACodeStoreError::UnexpectedError)
    }
    Ok(())
  }
  async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
    match self.codes.remove(&email) {
      Some(_) => Ok(()),
      None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
  }
  async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
    match self.codes.get(email) {
      Some((login_attempt_id, code)) => Ok((login_attempt_id.clone(), code.clone())),
      None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
  }
}

#[cfg(test)]
mod tests {
    use crate::domain::Email;
    use super::*;
    use uuid::Uuid;

  #[tokio::test]
  async fn test_add_remove_get_methods() {
    let email = Email::parse("email@email.com".to_owned()).expect("Failed to create email");
    let mut store = HashmapTwoFACodeStore {
      codes: HashMap::new()
    };
    let login_attempt_id = LoginAttemptId::parse(Uuid::new_v4().to_string()).expect("Failed to parse uuid");
    let two_fa_code = TwoFACode::parse("123456".to_owned()).expect("Failed to parse code");

    let inserted = store.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await;
    assert!(inserted.is_ok());
    assert_eq!(store.codes.is_empty(), false);

    let code = store.get_code(&email).await;
    assert!(code.is_ok());
    let (id, code) = code.unwrap();
    assert_eq!(id, login_attempt_id);
    assert_eq!(code, two_fa_code);
  }
}