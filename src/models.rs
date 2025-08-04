use crate::schema::license_keys;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::license_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LicenseKey {
    pub id: i32,
    pub key: String,
    pub expires: NaiveDateTime,
    pub is_activated: bool,
    pub hwid: Option<String>,
    pub description: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = license_keys)]
pub struct NewLicenseKey {
    pub key: String,
    pub hwid: Option<String>,
    pub description: Option<String>,
    pub expires: NaiveDateTime,
}
