use std::sync::Arc;

use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    Email,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    #[tracing::instrument(name = "add_code", skip_all)]
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let tuple = serde_json::to_string(&TwoFATuple(
            login_attempt_id.as_ref().to_string(),
            code.as_ref().to_string(),
        ))
        .wrap_err("Failed to json stringify tuple")
        .map_err(TwoFACodeStoreError::UnexpectedError)?;
        Ok(self
            .conn
            .write()
            .await
            .set_ex(&get_key(&email), tuple, TEN_MINUTES_IN_SECONDS)
            .wrap_err("Failed to set tuple in redis")
            .map_err(TwoFACodeStoreError::UnexpectedError)?)
    }
    #[tracing::instrument(name = "remove_code", skip_all)]
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(email);
        Ok(self
            .conn
            .write()
            .await
            .del(key)
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?)
    }
    #[tracing::instrument(name = "get_code", skip_all)]
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let key = get_key(email);
        match self.conn.write().await.get::<_, String>(key) {
            Ok(value) => {
                let data: TwoFATuple = serde_json::from_str(&value)
                    .wrap_err("Failed to parse tuple")
                    .map_err(TwoFACodeStoreError::UnexpectedError)?;

                let login_attempt_id =
                    LoginAttemptId::parse(data.0).map_err(TwoFACodeStoreError::UnexpectedError)?;

                let two_fa_code =
                    TwoFACode::parse(data.1).map_err(TwoFACodeStoreError::UnexpectedError)?;
                Ok((login_attempt_id, two_fa_code))
            }
            Err(_) => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

pub const TEN_MINUTES_IN_SECONDS: u64 = 600;
pub const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}
