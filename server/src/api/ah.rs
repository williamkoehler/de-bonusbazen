use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::*;
use tracing::*;

use crate::ArcState;

#[derive(Deserialize)]
struct Pagination {
    page: Option<usize>,
    size: Option<usize>,
}

async fn get_products(
    State(state): State<ArcState>,
    Query(pagination): Query<Pagination>,
) -> Result<
    Json<Vec<crate::databases::postgres::model::AhProduct>>,
    (StatusCode, Json<super::ErrorBody>),
> {
    let (page, size) = if let (Some(page), Some(size)) = (pagination.page, pagination.size) {
        (page, size)
    } else {
        (0, 100)
    };

    let products = state
        .postgres
        .ah_products(page, size)
        .await
        .map_err(|err| {
            error!("failed to get products: {}", err);
            super::ErrorReason::InternalError.into()
        })?;

    Ok(Json(products))
}

async fn get_products_most_bonus(
    State(state): State<ArcState>,
) -> Result<Json<Vec<crate::databases::postgres::model::AhProduct>>, (StatusCode, Json<super::ErrorBody>)> {
    let (page, size) = None.unwrap_or((0, 100));
    let products = state
        .postgres
        .ah_products_most_bonus(page, size)
        .await
        .map_err(|err| {
            error!("failed to get products with most bonus: {}", err);
            super::ErrorReason::InternalError.into()
        })?;

    Ok(Json(products))
}

#[derive(Debug, Deserialize)]
struct PostCommentRequestBody {
    product_id: i64,
    comment: String,
}

async fn post_comment(
    State(state): State<ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Json(request_body): Json<PostCommentRequestBody>,
) -> Result<(), (StatusCode, Json<super::ErrorBody>)> {
    return Err(super::ErrorReason::InternalError.into());
    // if auth_ext.rights >= UserRights::Normal {
    //     let post = state
    //         .ah_manager
    //         .add_post(
    //             visibility,
    //             auth_ext.id,
    //             &request_body.title,
    //             &request_body.body,
    //         )
    //         .await
    //         .map_err(|_| super::ErrorReason::InternalError.into())?;

    //     Ok(Json(post))
    // } else {
    //     Err(super::ErrorReason::Unauthorized.into())
    // }
}

async fn delete_comment(
    State(state): State<ArcState>,
    Extension(auth_ext): Extension<super::middleware::auth::AuthExtension>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, Json<super::ErrorBody>)> {
    return Err(super::ErrorReason::InternalError.into());
    // Ok(())
}

pub fn router() -> axum::Router<ArcState> {
    axum::Router::new()
        .route("/products", axum::routing::get(get_products))
        .route(
            "/products/most_bonus",
            axum::routing::get(get_products_most_bonus),
        )
}
