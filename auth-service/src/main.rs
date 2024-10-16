use std::{collections::HashMap, sync::Arc};
use auth_service::{services::HashmapUserStore, store::{AppState, UserStoreType}, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store: UserStoreType = Arc::new(RwLock::new(HashmapUserStore {
        users: HashMap::new()
    }));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app")
}
