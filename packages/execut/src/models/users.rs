use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub attendee: Attendee,
    pub recruiter: Recruiter,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub linkedin: Option<String>,
    pub study: Option<String>,
    pub degree: Option<String>,
    pub institution: Option<String>,
    pub graduation_year: Option<String>,
    pub company_slug: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attendee {
    pub linkedin: Option<String>,
    pub study: Option<String>,
    pub degree: Option<String>,
    pub institution: Option<String>,
    pub graduation_year: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recruiter {
    pub company_slug: Option<String>,
}

pub async fn fetch_user(pool: &PgPool, id: &Uuid) -> Result<User> {
    let user = sqlx::query_as!(
        UserModel,
        "select id
              , name
              , linkedin
              , study
              , degree
              , institution
              , graduation_year
              , company_slug
              , created_at
              , updated_at
           from users
          where id = $1
          limit 1",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| Error::Internal)?
    .ok_or_else(|| Error::UnknownUser)?;

    let UserModel {
        name,
        linkedin,
        study,
        degree,
        institution,
        graduation_year,
        company_slug,
        created_at,
        updated_at,
        ..
    } = user;

    let attendee = Attendee {
        linkedin,
        study,
        degree,
        institution,
        graduation_year,
    };
    let recruiter = Recruiter { company_slug };

    Ok(User {
        id: *id,
        name,
        attendee,
        recruiter,
        created_at,
        updated_at,
    })
}
