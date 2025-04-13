use axum::http::StatusCode;
use rand::{distr::Alphanumeric, Rng};
use uuid::Uuid;

use super::{
    encryption::{encrypt, get_new_encrypted_key, get_new_raw_key},
    internal_server_error, Store,
};

pub async fn join_family(
    user_uuid: &Uuid,
    family_join_code: String,
    store: &Store,
) -> Result<(), (StatusCode, String)> {
    let (family_uuid, correct_join_code) =
        store.get_family_from_join_code(&family_join_code).await?;

    if family_join_code != correct_join_code {
        return Err((StatusCode::UNAUTHORIZED, "Invalid Join Code".to_string()));
    }
    store
        .add_user_to_family(user_uuid, &family_uuid)
        .await
        .map_err(internal_server_error)?;
    Ok(())
}

pub async fn create_family(
    user_uuid: &Uuid,
    store: &Store,
) -> Result<String, (StatusCode, String)> {
    let families = store
        .get_family_uuid(user_uuid)
        .await
        .map_err(internal_server_error)?;
    if !families.is_empty() {
        return Err((
            StatusCode::FORBIDDEN,
            "Users with family cannot create new family".to_string(),
        ));
    }
    let family_join_code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let join_code_prefix = family_join_code
        .get(0..4)
        .ok_or(internal_server_error("Failed to creat join code prefix"))?;
    let encryption_key = get_new_raw_key().await;
    let join_code_encrypted =
        encrypt(&encryption_key, family_join_code.as_bytes()).map_err(internal_server_error)?;
    let fam_uuid = store
        .create_family(
            join_code_prefix.to_string(),
            join_code_encrypted,
            get_new_encrypted_key(Some(encryption_key))
                .await
                .map_err(internal_server_error)?,
        )
        .await
        .map_err(internal_server_error)?;
    store
        .add_user_to_family(user_uuid, &fam_uuid)
        .await
        .map_err(internal_server_error)?;
    Ok(family_join_code)
}

pub async fn get_family_join_code(
    user_uuid: &Uuid,
    store: &Store,
) -> Result<Option<String>, (StatusCode, String)> {
    store
        .get_family_join_code_from_user_uuid(user_uuid)
        .await
        .map_err(internal_server_error)
}
