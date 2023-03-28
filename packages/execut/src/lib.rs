pub mod config;
pub mod errors;
pub mod models;
pub mod routes;

pub mod extractors;

pub use config::Config;
pub use errors::{Error, Result};

use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

#[derive(Clone)]
pub struct Context {
    pub pool: PgPool,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}
