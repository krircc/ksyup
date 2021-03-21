use chrono::{Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role_id: i32,
    pub tanshi: i32,
    pub created_at: NaiveDateTime,
    pub avatar: String,
}