use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::*;
use tracing::*;

use crate::{ArcState, users::model};

#[derive(Debug, Serialize, Deserialize)]
struct PostUserRequestBody {
    name: String,
    nickname: Option<String>,
    email: Option<String>,
    password: String,
    rights: model::UserRights,
}

#[derive(Debug, Serialize, Deserialize)]
struct PatchUserRequestBody {
    name: Option<String>,
    nickname: Option<Option<String>>,
    email: Option<Option<String>>,
    password: Option<String>,
    rights: Option<model::UserRights>,
}

async fn get_users(
    State(state): State<crate::ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
) -> Result<Json<Vec<model::User>>, (StatusCode, Json<super::ErrorBody>)> {
    let mut include_normal = false;

    if auth_ext.rights >= model::UserRights::Normal {
        include_normal = true;
    }

    let users = state
        .user_manager
        .users(true, include_normal, false)
        .await
        .map_err(|_| super::ErrorReason::InternalError.into())?;

    Ok(Json(users))
}

async fn get_user(
    State(state): State<crate::ArcState>,
    Path(id): Path<i32>,
) -> Result<Json<model::User>, (StatusCode, Json<super::ErrorBody>)> {
    let user = state
        .user_manager
        .user(id)
        .await
        .map_err(|_| super::ErrorReason::InternalError.into())?;

    Ok(Json(user))
}

async fn post_user(
    State(state): State<crate::ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Json(request_body): Json<PostUserRequestBody>,
) -> Result<Json<model::User>, (StatusCode, Json<super::ErrorBody>)> {
    if auth_ext.rights >= model::UserRights::Admin {
        let user = state
            .user_manager
            .add_user(
                &request_body.name,
                request_body.nickname.as_deref(),
                request_body.email.as_deref(),
                &request_body.password,
                request_body.rights,
            )
            .await
            .map_err(|_| super::ErrorReason::InternalError.into())?;

        Ok(Json(user))
    } else {
        Err(super::ErrorReason::Unauthorized.into())
    }
}

async fn patch_user(
    State(state): State<crate::ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Path(id): Path<i32>,
    Json(request_body): Json<PatchUserRequestBody>,
) -> Result<(), (StatusCode, Json<super::ErrorBody>)> {
    if auth_ext.rights >= model::UserRights::Admin || auth_ext.id == id {
        state
            .user_manager
            .update_user(
                id,
                request_body.name.as_deref(),
                request_body.nickname.as_ref().map(|x| x.as_deref()),
                request_body.email.as_ref().map(|x| x.as_deref()),
                request_body.password.as_deref(),
                request_body.rights,
                None,
            )
            .await
            .map_err(|err| {
                error!("failed to update user: {}", err);
                super::ErrorReason::InternalError.into()
            })?;

        Ok(())
    } else {
        Err(super::ErrorReason::Unauthorized.into())
    }
}

async fn get_user_profile_picture(
    State(state): State<crate::ArcState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let profile_picture = state
        .user_manager
        .user_profile_picture(id)
        .await
        .map_err(|err| {
            error!("failed to update user profile picture: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });

    match profile_picture {
        Ok(Some(bytes)) => axum::http::Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "image/jpeg") // adjust for jpg, gif, etc.
            .body(axum::body::Body::from(bytes))
            .unwrap(),
        _ => axum::http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::from("No profile picture defined"))
            .unwrap(),
    }
}

async fn patch_user_profile_picture(
    State(state): State<crate::ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Path(id): Path<i32>,
    bytes: axum::body::Bytes,
) -> Result<(), (StatusCode, Json<super::ErrorBody>)> {
    if auth_ext.rights >= model::UserRights::Admin || auth_ext.id == id {
        state
            .user_manager
            .update_user(id, None, None, None, None, None, Some(&bytes))
            .await
            .map_err(|err| {
                error!("failed to update user profile picture: {}", err);
                super::ErrorReason::InternalError.into()
            })?;
        Ok(())
    } else {
        Err(super::ErrorReason::Unauthorized.into())
    }
}

async fn delete_user(
    State(state): State<crate::ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, Json<super::ErrorBody>)> {
    if auth_ext.rights >= model::UserRights::Admin {
        state
            .user_manager
            .remove_user(id)
            .await
            .map_err(|_| super::ErrorReason::InternalError.into())?;

        Ok(())
    } else {
        Err(super::ErrorReason::Unauthorized.into())
    }
}

pub fn router() -> axum::Router<ArcState> {
    axum::Router::new()
        .route("/", axum::routing::get(get_users).post(post_user))
        .route(
            "/{id}",
            axum::routing::get(get_user)
                .patch(patch_user)
                .delete(delete_user),
        )
        .route(
            "/{id}/profile_picture",
            axum::routing::get(get_user_profile_picture).patch(patch_user_profile_picture),
        )
}
