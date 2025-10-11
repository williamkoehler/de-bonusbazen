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
    ) -> error::Result<impl Iterator<Item = model::Product>> {
        Ok(self
            .postgres
            .ah_products_most_bonus()
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })?
            .into_iter()
            .map(|raw| raw.data))
    }

    pub async fn refresh_ah_products(&self) {
        let token = self.ah_service.authenticate().await.unwrap();
        let categories = self.ah_service.get_categories(&token).await.unwrap();

        for category in categories {
            match category.id {
                20603 | 1057 | 18519 | 18521 | 1165 | 11717 | 1045 => continue, // Skip non-food categories
                _ => {}
            }

            let mut page = 0;
            let mut page_count = 1;

            while page < page_count {
                info!("searching for ah products on page {}", page + 1,);
                let search_results = self
                    .ah_service
                    .search_products(&token, "", page, 500, Some(category.id))
                    .await
                    .unwrap();

                page_count = search_results.page.total_pages as usize;

                let mut raw_products = search_results
                    .products
                    .into_iter()
                    .map(|product| crate::databases::postgres::model::RawAhProduct {
                        id: product.hq_id as i64,
                        data: product.into(),
                    })
                    .filter(|raw| raw.data.bonus);

                self.postgres
                    .set_ah_products(&mut raw_products)
                    .await
                    .unwrap();

                page += 1;
            }
        }
        info!("refreshed AH products");

        // Ok(())
    }
}
