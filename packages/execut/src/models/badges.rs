use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{
        users::{fetch_user, UserModel},
        User,
    },
    Error, Result,
};

use super::users::{Attendee, Recruiter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    pub id: Uuid,
    pub badge_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scan {
    pub id: Uuid,
    pub initiator_id: Uuid,
    pub subject_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn fetch_user_with_badge(pool: &PgPool, badge: &String) -> Result<User> {
    let res = sqlx::query!(
        "select user_id
           from badges
          where badge_code = $1
          limit 1",
        badge
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| Error::Internal)?
    .ok_or_else(|| Error::UnknownBadge)?;

    let user = fetch_user(pool, &res.user_id).await;

    user
}

pub async fn fetch_scanned_users(pool: &PgPool, initiator_id: &Uuid) -> Result<Vec<User>> {
    let scanned_users = sqlx::query_as!(
        UserModel,
        "select u.id
              , u.name
              , u.linkedin
              , u.study
              , u.degree
              , u.institution
              , u.graduation_year
              , u.company_slug
              , u.created_at
              , u.updated_at
           from users as u, scans as s
          where s.subject_id = u.id
            and s.initiator_id = $1",
        initiator_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|_| Error::Internal)?;

    Ok(scanned_users
        .into_iter()
        .map(|user| {
            let UserModel {
                id,
                name,
                linkedin,
                study,
                degree,
                institution,
                graduation_year,
                company_slug,
                created_at,
                updated_at,
            } = user;

            let attendee = Attendee {
                linkedin,
                study,
                degree,
                institution,
                graduation_year,
            };
            let recruiter = Recruiter { company_slug };

            User {
                id,
                name,
                attendee,
                recruiter,
                created_at,
                updated_at,
            }
        })
        .collect())
}

pub async fn scan_badge(pool: &PgPool, initiator_id: &Uuid, subject_id: &Uuid) -> Result<()> {
    sqlx::query!(
        "insert into scans
           ( initiator_id
           , subject_id
           )
         values
           ( $1
           , $2
           )",
        initiator_id,
        subject_id,
    )
    .execute(pool)
    .await
    .map_err(|_| Error::Internal)?;

    Ok(())
}
