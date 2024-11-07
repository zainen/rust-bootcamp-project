use auth_service::{
    get_postgres_pool, services::{HashmapBannedTokenStore, HashmapTwoFACodeStore, HashmapUserStore, MockEmailClient}, store::{AppState, BannedTokenStoreType, EmailClientType, TwoFACodeStoreType, UserStoreType}, utils::constants::{prod, DATABASE_URL}, Application
};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
    let user_store: UserStoreType = Arc::new(RwLock::new(HashmapUserStore::default()));

    let banned_token_store: BannedTokenStoreType =
        Arc::new(RwLock::new(HashmapBannedTokenStore::default()));

    let two_fa_code_store: TwoFACodeStoreType =
        Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

    let email_client: EmailClientType = Arc::new(MockEmailClient);

    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app")
}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL).await.expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migration");

    pg_pool
}
