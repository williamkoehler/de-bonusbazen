use crate::{misc::ah::AhManager, misc::ah::model::AhProduct, services::ah::AhService};
use tracing::*;

#[derive(Clone)]
pub struct AhJobs {
    pub ah_manager: AhManager,
    pub ah_service: AhService,
}

impl AhJobs {
    pub fn new(ah_manager: AhManager, ah_service: AhService) -> Self {
        Self {
            ah_manager,
            ah_service,
        }
    }

    pub async fn update_ah_products_job(&self) -> anyhow::Result<()> {
        info!("updating AH products...");

        let token = self.ah_service.authenticate().await?;
        let categories = self.ah_service.get_categories(&token).await?;

        // Remove all old products
        if let Err(err) = self.ah_manager.remove_all_ah_products().await {
            error!("failed to remove old AH products: {}", err);
        }

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
                    .map::<AhProduct, _>(|product| product.into());

                self.ah_manager.set_ah_products(&mut raw_products).await?;

                page += 1;
            }
        }

        info!("successfully updated AH products");

        if let Err(err) = self.ah_manager.set_last_refresh(chrono::Utc::now()).await {
            error!("failed to update AH refresh time: {}", err);
        }

        Ok(())
    }
}
