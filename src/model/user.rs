use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Import Uuid from the uuid crate

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
