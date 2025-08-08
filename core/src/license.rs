use crate::models::{AuthRequest, AuthResponse};

use diesel::internal::derives::multiconnection::chrono;

use crate::repos::LicenseRepo;
use http::StatusCode;
use storage::models::{LicenseKey, NewLicenseKey};

pub struct LicenseService<R: LicenseRepo> {
    repo: R,
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
            if db_key.banned {
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
