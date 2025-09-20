use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewPassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct SignupInfo {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub(crate) struct EncryptedDataReturn {
    pub uuid: Option<Uuid>,
    pub owner_uuid: Uuid,
    pub encrypted_data: Vec<u8>,
    pub data_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Clone, Deserialize)]
pub(crate) struct EncryptedFamilyDataReturn {
    pub uuid: Option<Uuid>,
    pub family_uuid: Uuid,
    pub encrypted_data: Option<Vec<u8>>,
    pub data_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Item {
    pub name: String,
    pub price: f64,
    pub bought_for: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Transaction {
    pub input_for_family: Option<bool>,
    pub vendor: String,
    pub buyer: String,
    pub cost: f64,
    pub tags: Vec<String>,
    pub items: Vec<Item>, //TODO needs timestamp
    pub transaction_timestamp: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CachedData {
    pub uuid: Option<Uuid>,
    pub name: String,
    pub value: Value,
    pub created: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct VendorData {
    pub vendor: String,
    pub times_bought_from: i64,
    pub amount_spent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Goal {
    pub uuid: Option<Uuid>,
    pub name: String,
    pub value: String,
    pub created: Option<NaiveDateTime>,
}
