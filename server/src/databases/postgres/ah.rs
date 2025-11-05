use futures_util::TryStreamExt;
use sqlx::Row;

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
        let mut rows = sqlx::query("SELECT id, data FROM ah_products LIMIT $1 OFFSET $2")
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

    pub async fn ah_products_most_bonus(
        &self,
        page: usize,
        size: usize,
    ) -> error::Result<Vec<model::RawAhProduct>> {
        let mut products = Vec::new();

        let offset = (page * size) as i64;
        let size = size as i64;
        let mut rows = sqlx::query("SELECT id, data FROM ah_products WHERE ranking IS NOT NULL ORDER BY ranking DESC, id ASC LIMIT $1 OFFSET $2")
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
            let ranking = if let (Some(price_before_bonus), Some(price)) =
                (product.data.price_before_bonus, product.data.price)
            {
                Some(((price_before_bonus / price) * 1000.0) as i64)
            } else {
                None
            };

            sqlx::query("INSERT INTO ah_products (id, ranking, data) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET ranking = EXCLUDED.ranking, data = EXCLUDED.data")
                .bind(&product.id)
                .bind(&ranking)
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

    pub async fn add_ah_comment(
        &self,
        product_id: i64,
        user_id: i32,
        comment: &str,
    ) -> error::Result<()> {
        sqlx::query("INSERT INTO ah_comments (product_id, user_id, comment) VALUES ($1, $2, $3)")
            .bind(&product_id)
            .bind(&user_id)
            .bind(&comment)
            .execute(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(())
    }

    pub async fn remove_ah_comment(&self, id: i32) -> error::Result<()> {
        let result = sqlx::query("DELETE FROM ah_comments WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        if result.rows_affected() == 0 {
            return Err(error::Error::OperationFailed {
                msg: "failed to remove comment.",
            });
        }

        Ok(())
    }
}
