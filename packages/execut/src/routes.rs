use axum::Router;

use crate::Context;

mod register;
mod scans;

pub fn router() -> Router<Context> {
    let router = Router::new()
        .merge(register::router())
        .merge(scans::router());

    router
}
