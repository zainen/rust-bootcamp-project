use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if let Some(_) = self.users.get(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);

            Ok(())
        }
    }

    pub fn get_user(&self, email: &String) -> Result<&User, UserStoreError> {
        if let Some(found_user) = self.users.get(email) {
            Ok(found_user)
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    pub fn verify_user(&self, email: &String, password: &String) -> Result<(), UserStoreError> {
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
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user = User {
            email: "something".to_owned(),
            password: "password123".to_owned(),
            requires_2fa: false,
        };

        let result = store.add_user(user);

        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn get_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let email = "something".to_owned();

        let user = User {
            email: email.clone(),
            password: "password123".to_owned(),
            requires_2fa: false,
        };
        let inserted_user_result = store.add_user(user);

        assert_eq!(inserted_user_result.unwrap(), ());

        let found_user = store.get_user(&email);

        assert_eq!(found_user.unwrap().email, email)
    }

    #[tokio::test]
    async fn verify_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let email = "something".to_owned();
        let password = "password123".to_owned();

        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };

        let _inserted_user_result = store.add_user(user);

        let found_user = store.get_user(&email);
        let user = found_user.unwrap();
        assert_eq!(&user.email, &email);
        assert_eq!(&user.password, &password);


        let results = store.verify_user(&email, &password);


        assert_eq!(results.unwrap(), ());

    }
}
