use async_trait::async_trait;
use common::errors::internal_error;
use diesel::{ExpressionMethods, SelectableHelper};
use diesel::associations::HasTable;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use http::StatusCode;
use storage::models::{LicenseKey, NewLicenseKey};
use storage::repo::DatabaseRepo;
use storage::schema::license_keys::dsl::license_keys;
use storage::schema::license_keys::{hwid, is_activated, key};

#[async_trait]
pub trait LicenseRepo: Send + Sync {
    async fn get_key(&self, key_str: &str) -> anyhow::Result<LicenseKey, (StatusCode, String)>;
    async fn create(&self, req: NewLicenseKey) -> anyhow::Result<LicenseKey, (StatusCode, String)>;
    async fn activate_key(
        &self,
        lic_key: &str,
        hardware_id: &str,
    ) -> anyhow::Result<(), (StatusCode, String)>;
}

#[async_trait]
impl LicenseRepo for DatabaseRepo {
    async fn get_key(&self, key_str: &str) -> anyhow::Result<LicenseKey, (StatusCode, String)> {
        let mut conn = self.pool.get().await.map_err(internal_error)?;

        let result = license_keys
            .filter(key.eq(key_str))
            .first::<LicenseKey>(&mut conn)
            .await
            .map_err(internal_error)?;

        Ok(result)
    }

    async fn create(&self, req: NewLicenseKey) -> anyhow::Result<LicenseKey, (StatusCode, String)> {
        let mut conn = self.pool.get().await.map_err(internal_error)?;

        let created_key = diesel::insert_into(license_keys::table())
            .values(req)
            .returning(LicenseKey::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(internal_error)?;

        Ok(created_key)
    }
    async fn activate_key(
        &self,
        lic_key: &str,
        hardware_id: &str,
    ) -> anyhow::Result<(), (StatusCode, String)> {
        let mut conn = self.pool.get().await.map_err(internal_error)?;

        diesel::update(license_keys.filter(key.eq(lic_key)))
            .set((hwid.eq(hardware_id), is_activated.eq(true)))
            .execute(&mut conn)
            .await
            .map_err(internal_error)?;

        Ok(())
    }
}
