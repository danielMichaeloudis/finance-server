use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::RngCore;
use serde_json::Value;
use uuid::Uuid;

use super::store::Store;

pub async fn get_new_encrypted_key(key: Option<[u8; 32]>) -> Result<Vec<u8>, String> {
    dotenvy::dotenv().ok();

    let master_key_str = std::env::var("MASTER_ENCRYPTION_KEY")
        .map_err(|_| "Master Encryption Key Must Be Set".to_string())?;
    let master_key = master_key_str
        .as_bytes()
        .try_into()
        .map_err(|_| "Master Encryption Key Is Invalid".to_string())?;
    let key = match key {
        Some(key) => key,
        None => get_new_raw_key().await,
    };
    encrypt(master_key, &key)
}

pub async fn get_new_raw_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    key
}

async fn get_key(user_or_family: &str, uuid: &Uuid, store: &Store) -> Result<[u8; 32], String> {
    dotenvy::dotenv().ok();

    let master_key_str = std::env::var("MASTER_ENCRYPTION_KEY")
        .map_err(|_| "Master Encryption Key Must Be Set".to_string())?;
    let master_key = master_key_str
        .as_bytes()
        .try_into()
        .map_err(|_| "Master Encryption Key Is Invalid".to_string())?;

    let encrypted_key = match user_or_family {
        "user" => store
            .get_user_encryption_key(uuid)
            .await
            .map_err(|_| "Encryption key not found".to_string())?,
        "family" => store
            .get_family_encryption_key(uuid)
            .await
            .map_err(|_| "Encryption key not found".to_string())?,
        _ => panic!("Invalid Input"),
    };
    let decrypted_key = decrypt(master_key, encrypted_key).map_err(|mut e| {
        e.push_str("\nError occured while decrypting key");
        e
    })?;
    if decrypted_key.len() == 32 {
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&decrypted_key);
        Ok(key_array)
    } else {
        Err("Invalid decrypted key length".to_string())
    }
}

pub fn encrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce);
    match cipher.encrypt(Nonce::from_slice(&nonce), data) {
        Ok(mut ciphertext) => {
            let mut result = nonce.to_vec();
            result.append(&mut ciphertext);
            Ok(result)
        }
        Err(e) => Err(format!("Encryption failed with error: {}", e)),
    }
}

fn decrypt(key: &[u8; 32], encrypted_data: Vec<u8>) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let (nonce, cipher_data) = encrypted_data.split_at(12);

    match cipher.decrypt(Nonce::from_slice(nonce), cipher_data) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Decryption Failed with error: {:?}", e)),
    }
}

pub async fn encrypt_data(
    user_or_family: &str,
    uuid: &Uuid,
    data: Value,
    store: &Store,
) -> Result<Vec<u8>, String> {
    let key = get_key(user_or_family, uuid, store).await?;
    let data_str = data.to_string();
    let data_u8 = data_str.as_bytes();
    encrypt(&key, data_u8).map_err(|mut e| {
        e.push_str("\nError occured while encrypting data");
        e
    })
}

pub async fn decrypt_data(
    user_or_family: &str,
    uuid: &Uuid,
    encrypted_data: Vec<u8>,
    store: &Store,
) -> Result<Value, String> {
    let key = get_key(user_or_family, uuid, store).await?;
    let decypted_data = decrypt(&key, encrypted_data).map_err(|mut e| {
        e.push_str("\nError occured while decrypting data");
        e
    })?;
    serde_json::from_slice(&decypted_data).map_err(|e| e.to_string())
}

pub async fn decrypt_string(
    user_or_family: &str,
    uuid: &Uuid,
    encrypted_data: Vec<u8>,
    store: &Store,
) -> Result<String, String> {
    let key = get_key(user_or_family, uuid, store).await?;
    let decypted_data = decrypt(&key, encrypted_data).map_err(|mut e| {
        e.push_str("\nError occured while decrypting data");
        e
    })?;
    String::from_utf8(decypted_data).map_err(|e| e.to_string())
}
