use axum::{extract::State, routing::post, Json, Router};
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use uuid::Uuid;

use crate::{Context, Error, Result, extractors::Claims};

#[derive(Deserialize)]
struct GeneratePayload {
    id: Option<Uuid>,
    name: String,
    linkedin: Option<String>,
    study: Option<String>,
    degree: Option<String>,
    institution: Option<String>,
    graduation_year: Option<String>,
    company_slug: Option<String>,
    badges: Vec<String>,
}

async fn register(
    State(context): State<Context>,
    Json(payload): Json<GeneratePayload>,
) -> Result<String> {
    let Context {
        pool, encoding_key, ..
    } = context;

    let GeneratePayload {
        id,
        name,
        linkedin,
        study,
        degree,
        institution,
        graduation_year,
        company_slug,
        badges,
    } = payload;

    let id = if let Some(id) = id {
        id
    } else {
        Uuid::new_v4()
    };

    let rec = sqlx::query!(
        "insert into users
           ( id
           , name
           , linkedin
           , study
           , degree
           , institution
           , graduation_year
           , company_slug
           )
         values
           ( $1
           , $2
           , $3
           , $4
           , $5
           , $6
           , $7
           , $8
           )
         returning id",
        id,
        name,
        linkedin,
        study,
        degree,
        institution,
        graduation_year,
        company_slug
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| Error::Internal)?;

    let id = rec.id;

    for badge in badges {
        sqlx::query!(
            "insert into badges
               ( user_id
               , badge_code
               )
             values
               ( $1
               , $2
               )",
            id,
            badge)
            .execute(&pool)
            .await
            .map_err(|_| Error::Internal)?;
    }

    let claims = Claims {
        subject: id,
        expires_at: 1680022800,
    };

    let token = encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| Error::MissingCredentials)?;

    Ok(token)
}

pub(crate) fn router() -> Router<Context> {
    let router = Router::new()
        // `GET /register`
        .route("/register", post(register));

    router
}
