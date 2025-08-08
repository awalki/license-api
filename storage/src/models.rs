use crate::schema::{license_keys, user_info};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = license_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LicenseKey {
    pub id: i32,
    pub key: String,
    pub expires: NaiveDateTime,
    pub is_activated: bool,
    pub banned: bool,
    pub hwid: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = license_keys)]
pub struct NewLicenseKey {
    pub expires: NaiveDateTime,
    pub key: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = user_info)]
#[diesel(primary_key(license_id))]
#[diesel(belongs_to(LicenseKey, foreign_key = license_id))]
pub struct UserInfo {
    pub license_id: i32,
    pub first_login: String,
    pub last_login: String,
    pub last_ip: IpNet,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub cpu_info: Option<String>,
    pub cpu_cores: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = user_info)]
pub struct NewUserInfo {
    pub license_id: i32,
    pub first_login: String,
    pub last_login: String,
    pub last_ip: IpNet,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub cpu_info: Option<String>,
    pub cpu_cores: Option<i32>,
    pub notes: Option<String>,
}
