use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if let Some(_) = self.users.get(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);

            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn verify_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if &user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use secrecy::Secret;

    use crate::domain::Password;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user = User {
            email: Email::parse(Secret::new("ok@email.com".to_owned())).unwrap(),
            password: Password::parse(Secret::new("longenough".to_owned())).unwrap(),
            requires_2fa: false,
        };

        let result = store.add_user(user).await;

        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn get_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };
        let user = User {
            email: Email::parse(Secret::new("ok@email.com".to_owned())).unwrap(),
            password: Password::parse(Secret::new("longenough".to_owned())).unwrap(),
            requires_2fa: false,
        };
        let inserted_user_result = store.add_user(user);

        assert_eq!(inserted_user_result.await.unwrap(), ());

        // keep clone off of the User Struct
        let user = User {
            email: Email::parse(Secret::new("ok@email.com".to_owned())).unwrap(),
            password: Password::parse(Secret::new("longenough".to_owned())).unwrap(),
            requires_2fa: false,
        };

        let found_user = store.get_user(&user.email);

        let user = User {
            email: Email::parse(Secret::new("ok@email.com".to_owned())).unwrap(),
            password: Password::parse(Secret::new("longenough".to_owned())).unwrap(),
            requires_2fa: false,
        };

        assert_eq!(found_user.await.unwrap().email, user.email)
    }

    #[tokio::test]
    async fn verify_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user = User {
            email: Email::parse(Secret::new("ok@email.com".to_owned())).unwrap(),
            password: Password::parse(Secret::new("longenough".to_owned())).unwrap(),
            requires_2fa: false,
        };

        let _inserted_user_result = store.add_user(user).await.unwrap();

        let user = User {
            email: Email::parse(Secret::new("ok@email.com".to_owned())).unwrap(),
            password: Password::parse(Secret::new("longenough".to_owned())).unwrap(),
            requires_2fa: false,
        };

        let found_user = store.get_user(&user.email);
        let user = User {
            email: Email::parse(Secret::new("ok@email.com".to_owned())).unwrap(),
            password: Password::parse(Secret::new("longenough".to_owned())).unwrap(),
            requires_2fa: false,
        };
        let found_user = found_user.await.unwrap();

        assert_eq!(found_user.email, user.email);
        assert_eq!(found_user.password, user.password);

        let results = store.verify_user(&user.email, &user.password);

        assert_eq!(results.await.unwrap(), ());
    }
}
