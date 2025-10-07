use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::*;
use tracing::*;

use crate::{ArcState, posts::model::PostVisibility, users::model::Rights};

#[derive(Debug, Deserialize)]
struct PostPostRequestBody {
    visibility: Option<crate::posts::model::PostVisibility>,
    title: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PatchPostRequestBody {
    visibility: Option<crate::posts::model::PostVisibility>,
    title: Option<String>,
    body: Option<String>,
}

async fn get_posts(
    State(state): State<ArcState>,
) -> Result<Json<Vec<crate::posts::model::Post>>, StatusCode> {
    let posts = state.post_manager.posts().await.map_err(|err| {
        error!("failed to get posts: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(posts))
}

async fn get_post(
    State(state): State<ArcState>,
    Path(id): Path<i32>,
) -> Result<Json<crate::posts::model::Post>, StatusCode> {
    let post = state
        .post_manager
        .post(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}

async fn post_post(
    State(state): State<ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Json(request_body): Json<PostPostRequestBody>,
) -> Result<Json<crate::posts::model::Post>, StatusCode> {
    if auth_ext.rights >= Rights::Member {
        let visibility = request_body
            .visibility
            .unwrap_or(crate::posts::model::PostVisibility::Draft);

        if visibility >= PostVisibility::Visible {
            if !(auth_ext.rights >= Rights::Maintainer) {
                warn!("user with insufficient rights tried to create a visible post");
                return Err(StatusCode::FORBIDDEN);
            }
        }

        let post = state
            .post_manager
            .add_post(
                visibility,
                auth_ext.id,
                &request_body.title,
                &request_body.body,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(post))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

async fn patch_post(
    State(state): State<ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Path(id): Path<i32>,
    Json(request_body): Json<PatchPostRequestBody>,
) -> Result<(), StatusCode> {
    if auth_ext.rights >= Rights::Member {
        // Check user rights for visibility change
        let visibility = if let Some(visibility) = request_body.visibility {
            if visibility >= PostVisibility::Visible {
                if !(auth_ext.rights >= Rights::Maintainer) {
                    warn!("user with insufficient rights tried to create a visible post");
                    return Err(StatusCode::FORBIDDEN);
                }
            }
            Some(visibility)
        } else {
            None
        };

        state
            .post_manager
            .update_post(
                id,
                visibility,
                request_body.title.as_deref(),
                request_body.body.as_deref(),
            )
            .await
            .map_err(|err| {
                error!("failed to update post: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

async fn delete_post(
    State(state): State<ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    if auth_ext.rights >= Rights::Maintainer {
        state
            .post_manager
            .remove_post(id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub fn router() -> axum::Router<ArcState> {
    axum::Router::new()
        .route("/", axum::routing::get(get_posts).post(post_post))
        .route(
            "/{id}",
            axum::routing::get(get_post)
                .patch(patch_post)
                .delete(delete_post),
        )
}
