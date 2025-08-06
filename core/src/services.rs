use crate::models::{AuthRequest, AuthResponse};
use async_trait::async_trait;
use common::errors::internal_error;
use diesel::ExpressionMethods;
use diesel::associations::HasTable;
use diesel::internal::derives::multiconnection::chrono;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use http::StatusCode;
use storage::models::{LicenseKey, NewLicenseKey};
use storage::repo::DatabaseRepo;
use storage::schema::license_keys::dsl::*;

#[async_trait]
pub trait LicenseRepo: Send + Sync {
    async fn create(&self, req: NewLicenseKey) -> anyhow::Result<LicenseKey, (StatusCode, String)>;
    async fn get_key(&self, key: &str) -> anyhow::Result<LicenseKey, (StatusCode, String)>;
    async fn activate_key(
        &self,
        lic_key: &str,
        hardware_id: &str,
    ) -> anyhow::Result<(), (StatusCode, String)>;
}

pub struct LicenseService<R: LicenseRepo> {
    repo: R,
}

#[async_trait]
impl LicenseRepo for DatabaseRepo {
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
    async fn get_key(&self, lic_key: &str) -> anyhow::Result<LicenseKey, (StatusCode, String)> {
        let mut conn = self.pool.get().await.map_err(internal_error)?;

        let db_key: LicenseKey = license_keys
            .filter(key.eq(lic_key))
            .select(LicenseKey::as_select())
            .first::<LicenseKey>(&mut conn)
            .await
            .map_err(internal_error)?;

        Ok(db_key)
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

impl<R: LicenseRepo> LicenseService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn generate(
        &self,
        req: NewLicenseKey,
    ) -> anyhow::Result<LicenseKey, (StatusCode, String)> {
        if let Some(_) = req.key {
            let lic = self.repo.create(req).await?;

            return Ok(lic);
        }
        let new_key = NewLicenseKey {
            key: Some(common::license::generate_license_key()),
            expires: req.expires,
        };
        let lic = self.repo.create(new_key).await?;

        Ok(lic)
    }

    pub async fn auth(
        &self,
        req: AuthRequest,
    ) -> anyhow::Result<AuthResponse, (StatusCode, String)> {
        let db_key = self.repo.get_key(&*req.key).await?;

        if let Some(hwid_str) = db_key.hwid {
            if req.hwid != hwid_str {
                return Err((StatusCode::UNAUTHORIZED, "unauthorized".into()));
            }
            if db_key.expires < chrono::Utc::now().naive_utc() {
                return Err((StatusCode::UNAUTHORIZED, "unauthorized".into()));
            }
        } else {
            self.repo.activate_key(&*req.key, &*req.hwid).await?;
        }

        Ok(AuthResponse {
            message: "key is valid".into(),
        })
    }
}
