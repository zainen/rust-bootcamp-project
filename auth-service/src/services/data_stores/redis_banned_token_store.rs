use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};
#[derive(Clone)]
pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let banned_key = get_key(&token);

        let value = true;

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        let _: () = self
            .conn
            .write()
            .await
            .set_ex::<_, _, ()>(banned_key, value, ttl)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn verify_token_exists(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self
            .conn
            .write()
            .await
            .get::<&str, bool>(&get_key(token))
            .map_err(|_| BannedTokenStoreError::TokenNotFound)?
        )
    }
}

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
