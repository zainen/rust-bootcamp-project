use auth_service::{
    services::{HashmapBannedTokenStore, HashmapTwoFACodeStore, HashmapUserStore},
    store::{AppState, BannedTokenStoreType, TwoFACodeStoreType, UserStoreType},
    utils::constants::prod,
    Application,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store: UserStoreType = Arc::new(RwLock::new(HashmapUserStore {
        users: HashMap::new(),
    }));
    let banned_token_store: BannedTokenStoreType = Arc::new(RwLock::new(HashmapBannedTokenStore {
        tokens: HashMap::new(),
    }));
    let two_fa_code_store: TwoFACodeStoreType = Arc::new(RwLock::new(HashmapTwoFACodeStore {
        codes: HashMap::new(),
    }));
    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app")
}
