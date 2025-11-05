use crate::{
    databases::{
        postgres::{PostgresDb, model::AhProduct},
        redis::RedisDb,
    },
    services::ah::AhService,
};
use tracing::*;

#[derive(Clone)]
pub struct AhJobs {
    postgres: PostgresDb,
    redis: RedisDb,
    ah_service: AhService,
}

impl AhJobs {
    pub fn new(postgres: PostgresDb, redis: RedisDb, ah_service: AhService) -> Self {
        Self {
            postgres,
            redis,
            ah_service,
        }
    }

    pub async fn update_ah_products_job(&self) -> anyhow::Result<()> {
        info!("updating AH products...");

        let token = self.ah_service.authenticate().await?;
        let categories = self.ah_service.get_categories(&token).await?;

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
                    .await?;

                page_count = search_results.page.total_pages as usize;

                let mut raw_products = search_results
                    .products
                    .into_iter()
                    .map::<AhProduct, _>(|product| product.into())
                    .filter(|product| product.bonus);

                self.postgres.set_ah_products(&mut raw_products).await?;

                page += 1;
            }
        }

        info!("successfully updated AH products");

        if let Err(err) = self.redis.set_last_ah_refresh(chrono::Utc::now()).await {
            error!("failed to update AH refresh time: {}", err);
        }

        Ok(())
    }
}
