use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::{
    extractors::Claims,
    models::{badges, User},
    Context, Result,
};

async fn all_scans(claim: Claims, State(context): State<Context>) -> Result<Json<Vec<User>>> {
    // Extract `subject` from JWT token
    let Claims { subject, .. } = claim;

    let Context { pool, .. } = context;

    let users = badges::fetch_scanned_users(&pool, &subject).await?;

    Ok(Json(users))
}

async fn scan_badge(
    claim: Claims,
    Path(badge): Path<String>,
    State(context): State<Context>,
) -> Result<Json<User>> {
    // Extract `subject` from JWT token
    let Claims { subject, .. } = claim;

    let Context { pool, .. } = context;

    let user = badges::fetch_user_with_badge(&pool, &badge).await?;

    crate::models::badges::scan_badge(&pool, &subject, &user.id).await?;

    Ok(Json(user))
}

pub(crate) fn router() -> Router<Context> {
    let router = Router::new()
        // `GET /scans`
        .route("/scans", get(all_scans))
        // `POST /scans`
        .route("/scans/:badge", post(scan_badge));

    router
}
