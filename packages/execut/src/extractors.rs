use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Context, Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub subject: Uuid,

    #[serde(rename = "exp")]
    pub expires_at: usize,
}

#[async_trait]
impl FromRequestParts<Context> for Claims {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &Context) -> Result<Self> {
        let Context { decoding_key, .. } = state;

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::InvalidToken)?;

        let TokenData { claims, .. } =
            decode(bearer.token(), &decoding_key, &Validation::default())
                .map_err(|_| Error::InvalidToken)?;

        Ok(claims)
    }
}
