use std::{collections::HashMap, sync::Arc};
use auth_service::{services::HashmapUserStore, store::{AppState, UserStoreType}, utils::constants::prod, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store: UserStoreType = Arc::new(RwLock::new(HashmapUserStore {
        users: HashMap::new()
    }));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app")
}
