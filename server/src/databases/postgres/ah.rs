use futures_util::TryStreamExt;
use sqlx::{Row};

use super::error;
use crate::databases::postgres::model;

impl super::PostgresDb {
    pub fn ah_product_count(&self) -> error::Result<i64> {
        todo!();
    }

    pub async fn ah_products(
        &self,
        page: usize,
        size: usize,
    ) -> error::Result<Vec<model::RawAhProduct>> {
        let mut products = Vec::new();

        let offset = (page * size) as i64;
        let size = size as i64;
        let mut rows = 
            sqlx::query("SELECT id, data FROM ah_products LIMIT $1 OFFSET $2")
                .bind(&size)
                .bind(&offset)
                .fetch(&self.pool);
        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?
        {
            let id: i64 = row
                .try_get("id")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let data: sqlx::types::Json<crate::misc::ah::model::Product> = row
                .try_get("data")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            products.push(model::RawAhProduct { id, data: data.0 });
        }

        Ok(products)
    }

    pub async fn ah_products_most_bonus(&self) -> error::Result<Vec<model::RawAhProduct>> {
        let mut products = Vec::new();

        let mut rows = 
            sqlx::query("SELECT id, data FROM ah_products WHERE data::jsonb ? 'price_before_bonus' AND data::jsonb ? 'price' ORDER BY (cast(data::jsonb -> 'price_before_bonus' as real) / cast(data::jsonb -> 'price' as real)) DESC LIMIT 40")
                .fetch(&self.pool);
        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?
        {
            let id: i64 = row
                .try_get("id")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let data: sqlx::types::Json<crate::misc::ah::model::Product> = row
                .try_get("data")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            products.push(model::RawAhProduct { id, data: data.0 });
        }

        Ok(products)
    }

    pub async fn set_ah_products(
        &self,
        products: &mut impl Iterator<Item = model::RawAhProduct>,
    ) -> error::Result<()> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        for product in products {
            sqlx::query("INSERT INTO ah_products (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data")
                .bind(&product.id)
                .bind(sqlx::types::Json(&product.data))
                .execute(&mut *transaction)
                .await
                .map_err(|err| error::Error::SqlxError { inner: err })?;
        }

        transaction
            .commit()
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(())
    }
}
