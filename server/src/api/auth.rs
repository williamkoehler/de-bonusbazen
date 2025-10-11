use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::*;
use tracing::*;

use crate::users::{helper::JwtClaims, model::Rights};

#[derive(Debug, Deserialize)]
pub struct PostLoginRequestBody {
    name: String,
    password: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PostLoginResponseBody {
    token: String,
    id: i32,
    nickname: Option<String>,
    email: Option<String>,
    rights: crate::users::model::Rights,
}

pub async fn login(
    State(state): State<crate::ArcState>,
    Json(req_body): Json<PostLoginRequestBody>,
) -> Result<Json<PostLoginResponseBody>, StatusCode> {
    let (user, hash) = state
        .user_manager
        .user_and_hash_by_name(&req_body.name)
        .await
        .map_err(|err| {
            warn!(name = req_body.name, "user does not exist: {}", err);
            StatusCode::UNAUTHORIZED
        })?;

    if user.rights() == Rights::Unauthenticated {
        warn!(
            name = req_body.name,
            "user tried to login without verifying email"
        );
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !crate::users::helper::verify_hash(&hash, &req_body.password) {
        warn!(
            name = req_body.name,
            "user tried to login with invalid password"
        );
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = crate::users::helper::generate_jwt(
        &user,
        state.config.jwt_expiry_time,
        &state.config.jwt_authentication_secret,
    )
    .map_err(|err| {
        error!(id = user.id(), "failed to generate jwt: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PostLoginResponseBody {
        token,
        id: user.id(),
        nickname: user.nickname().map(|x| x.to_string()),
        email: user.email().map(|x| x.to_string()),
        rights: user.rights(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct PostRegisterRequestBody {
    name: String,
    nickname: Option<String>,
    email: String,
    password: String,
}

pub async fn post_register(
    State(state): State<crate::ArcState>,
    Json(req_body): Json<PostRegisterRequestBody>,
) -> Result<(), StatusCode> {
    let user = state
        .user_manager
        .add_user(
            &req_body.name,
            req_body.nickname.as_deref(),
            Some(&req_body.email),
            &req_body.password,
            Rights::Unauthenticated,
        )
        .await
        .map_err(|err| {
            warn!(name = req_body.name, "failed to add user: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Add verification
    state
        .postgres
        .add_verification(user.id())
        .await
        .map_err(|err| {
            warn!(id = user.id(), "failed to add verification: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let token = {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|err| {
                error!(id = user.id(), "time went backwards: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .as_secs();

        let claims = JwtClaims {
            id: user.id(),
            rights: user.rights(),
            exp: (now + 3600) as usize,
        };

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(state.config.jwt_verification_secret.as_ref()),
        )
        .map_err(|err| {
            error!(
                id = user.id(),
                "failed to generate verification jwt: {}", err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    };

    info!("register user with {}", format!("/api/register/{}", token));

    Ok(())
}

pub async fn get_verify(
    State(state): State<crate::ArcState>,
    Path(token): Path<String>,
) -> Result<(), StatusCode> {
    // Verify JWT
    let claims = {
        jsonwebtoken::decode::<JwtClaims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_verification_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .claims
    };

    let verification = state
        .postgres
        .has_verification(claims.id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    if !verification {
        warn!(
            id = claims.id,
            "user tried to verify without a pending verification"
        );
        return Err(StatusCode::NOT_FOUND);
    }

    // Update user rights to normal
    state
        .user_manager
        .update_user(
            claims.id,
            None,
            None,
            None,
            None,
            Some(Rights::Normal),
            None,
        )
        .await
        .map_err(|err| {
            error!(id = claims.id, "failed to set user rights: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Remove verification
    if let Err(err) = state.postgres.remove_verification(claims.id).await {
        error!(id = claims.id, "failed to remove verification: {}", err);
    }

    Ok(())
}
