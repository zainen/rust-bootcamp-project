use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let hashed_password = compute_password_hash(user.password.as_ref().to_string())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
        "#,
            &user.email.as_ref().to_string(),
            &hashed_password,
            &user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!(
            r#"
            SELECT email, password_hash, requires_2fa 
            FROM users 
            WHERE email = $1;
            "#,
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?
        .map(|row| {
            Ok(User {
                email: Email::parse(row.email).map_err(|_| UserStoreError::UnexpectedError)?,
                password: Password::parse(row.password_hash)
                    .map_err(|_| UserStoreError::UnexpectedError)?,
                requires_2fa: row.requires_2fa,
            })
        })
        .ok_or(UserStoreError::UserNotFound)?
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn verify_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        let user = match self.get_user(email).await {
            Err(_) => return Err(UserStoreError::InvalidCredentials),
            Ok(user) => user,
        };
        verify_password_hash(
            user.password.as_ref().to_string(),
            password.as_ref().to_string(),
        )
        .await
        .map_err(|_| UserStoreError::InvalidCredentials)?;
        Ok(())
    }
}

    #[tracing::instrument(name = "Validating password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let result = tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash)?;

        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected_password_hash)
            .map_err(|e| e.into())
    })
    .await;

    result?
}

    #[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let resp: Result<String, Box<dyn Error + Send + Sync>> =
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut rand::thread_rng()); // Use OsRng for secure randomness

            let params = Params::new(15_000, 2, 1, None).map_err(|e| {
                // Convert any error to Box<dyn Error>
                Box::<dyn Error + Send + Sync>::from(e)
            })?;

            let password_hash = Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| Box::<dyn Error + Send + Sync>::from(e))? // Handle hashing error and convert
                .to_string();

            Ok(password_hash)
        })
        .await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e))?; // Handle potential join error

    resp
}
