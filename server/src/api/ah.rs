use axum::{
    Json,
    extract::{Query, State},
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
) -> Result<Json<Vec<crate::misc::ah::model::Product>>, (StatusCode, Json<super::ErrorBody>)> {
    let pagination = if let (Some(page), Some(size)) = (pagination.page, pagination.size) {
        Some((page, size))
    } else {
        None
    };

    let products = state
        .ah_manager
        .ah_products(pagination)
        .await
        .map_err(|err| {
            error!("failed to get products: {}", err);
            super::ErrorReason::InternalError.into()
        })?;

    Ok(Json(products.collect::<Vec<_>>()))
}

async fn get_products_most_bonus(
    State(state): State<ArcState>,
) -> Result<Json<Vec<crate::misc::ah::model::Product>>, (StatusCode, Json<super::ErrorBody>)> {
    let products = state
        .ah_manager
        .ah_products_most_bonus()
        .await
        .map_err(|err| {
            error!("failed to get products with most bonus: {}", err);
            super::ErrorReason::InternalError.into()
        })?;

    Ok(Json(products.collect::<Vec<_>>()))
}

pub fn router() -> axum::Router<ArcState> {
    axum::Router::new()
        .route("/products", axum::routing::get(get_products))
        .route(
            "/products/most_bonus",
            axum::routing::get(get_products_most_bonus),
        )
}
