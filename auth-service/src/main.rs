use auth_service::{
    get_postgres_pool, get_redis_client,
    services::{
        MockEmailClient, PostgresUserStore,
        RedisBannedTokenStore, RedisTwoFACodeStore,
    },
    store::{AppState, BannedTokenStoreType, EmailClientType, TwoFACodeStoreType, UserStoreType},
    utils::{constants::{prod, DATABASE_URL, REDIS_HOST_NAME}, tracing::init_tracing},
    Application,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    init_tracing();
    let pg_pool = configure_postgresql().await;
    let redis_connection = Arc::new(RwLock::new(configure_redis()));
    let user_store: UserStoreType = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));

    let banned_token_store: BannedTokenStoreType = Arc::new(RwLock::new(
        RedisBannedTokenStore::new(redis_connection.clone()),
    ));

    let two_fa_code_store: TwoFACodeStoreType =
        Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));

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
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migration");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connections")
}
