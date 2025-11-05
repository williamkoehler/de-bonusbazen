use tracing::*;

use crate::services::ah::AhService;

pub mod error;
pub mod model;

pub struct AhManager {
    ah_service: AhService,
    postgres: crate::databases::postgres::PostgresDb,
}

impl AhManager {
    pub fn new(ah_service: AhService, postgres: crate::databases::postgres::PostgresDb) -> Self {
        Self {
            ah_service,
            postgres,
        }
    }

    pub async fn ah_products(
        &self,
        pagination: Option<(usize, usize)>,
    ) -> error::Result<impl Iterator<Item = model::Product>> {
        let (page, size) = pagination.unwrap_or((0, 100));
        Ok(self
            .postgres
            .ah_products(page, size)
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })?
            .into_iter()
            .map(|raw| raw.data))
    }

    pub async fn ah_products_most_bonus(
        &self,
        pagination: Option<(usize, usize)>,
    ) -> error::Result<impl Iterator<Item = model::Product>> {
        let (page, size) = pagination.unwrap_or((0, 100));
        Ok(self
            .postgres
            .ah_products_most_bonus(page, size)
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })?
            .into_iter()
            .map(|raw| raw.data))
    }
}
