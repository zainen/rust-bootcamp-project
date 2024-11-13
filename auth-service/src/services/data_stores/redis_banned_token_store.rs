use std::sync::Arc;

use color_eyre::eyre::{Context, Result};
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
    #[tracing::instrument(name = "add_token", skip_all)]
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let banned_key = get_key(&token);

        let value = true;

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("Failed to cast TOKEN_TTL_SECONDS to u64")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        let _: () = self
            .conn
            .write()
            .await
            .set_ex::<_, _, ()>(banned_key, value, ttl)
            .wrap_err("Failed to set banned token in redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;
        Ok(())
    }

    #[tracing::instrument(name = "verify_token_exists", skip_all)]
    async fn verify_token_exists(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self
            .conn
            .write()
            .await
            .exists(&get_key(token))
            .wrap_err("Failed to get token from redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?)
    }
}

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
