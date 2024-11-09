use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;


lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref REDIS_HOST_NAME: String = set_redis_host();
}

fn set_token() -> String {
    dotenv().ok();
    std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set")

}

fn set_database_url() -> String {
    dotenv().ok();
    std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set")

}

fn set_redis_host() -> String {
    dotenv().ok();
    std_env::var(env::REDIS_HOST_NAME_ENV_VAR).expect("REDIS_HOST_NAME must be set")
}


pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL"; 
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}   

