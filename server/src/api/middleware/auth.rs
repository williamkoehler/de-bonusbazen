use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{ArcState, databases::postgres::model};

#[derive(Clone)]
pub struct AuthExtension {
    pub id: i32,
    pub rights: model::UserRights,
}

pub async fn auth_middleware(
    State(state): State<ArcState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract authorization header
    let parts = request.headers();
    let header = parts
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_ext = match header {
        Some(header) => {
            let token = if let Some(token) = header.strip_prefix("Bearer ") {
                token
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            };

            let claims =
                crate::users::helper::verify_jwt(token, &state.config.jwt.authentication_secret)
                    .map_err(|_| StatusCode::UNAUTHORIZED)?;

            AuthExtension {
                id: claims.id,
                rights: claims.rights,
            }
        }
        None => AuthExtension {
            id: 0,
            rights: model::UserRights::Unauthenticated,
        },
    };

    // Add authentication extension
    request.extensions_mut().insert(auth_ext);

    Ok(next.run(request).await)
}
