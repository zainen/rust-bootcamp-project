use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

fn set_token() -> String {
    dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set");

    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty");
    }
    secret
}

fn set_database_url() -> String {
    dotenv().ok();
    let database_url = std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set");

    if database_url.is_empty() {
        panic!("DATABASE_URL must not be empty");
    }
    database_url
}

lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL"; 
}

pub const JWT_COOKIE_NAME: &str = "jwt";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}   

lazy_static! {
    pub static ref DATABASE_URL: String = set_database_url();
}
