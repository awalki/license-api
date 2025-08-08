use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use http::StatusCode;
use common::errors::internal_error;
use crate::models::LicenseKey;
use crate::Pool;
use crate::schema::license_keys::dsl::license_keys;
use crate::schema::license_keys::key;

pub struct DatabaseRepo {
    pub pool: Pool,
}

impl DatabaseRepo {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_key(&self, lic_key: &str) -> anyhow::Result<LicenseKey, (StatusCode, String)> {
        let mut conn = self.pool.get().await.map_err(internal_error)?;

        let db_key: LicenseKey = license_keys
            .filter(key.eq(lic_key))
            .select(LicenseKey::as_select())
            .first::<LicenseKey>(&mut conn)
            .await
            .map_err(internal_error)?;

        Ok(db_key)
    }
}
