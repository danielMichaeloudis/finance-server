use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    models::{LoginInfo, NewPassword, SignupInfo},
    utils::{get_uuid_from_token, internal_server_error, JWTKeyProvider, Store},
};

pub async fn route_get_has_family(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<bool>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    let res = store
        .get_family_uuid(&user_uuid)
        .await
        .map_err(internal_server_error)?;

    Ok(Json(!res.is_empty()))
}

pub async fn route_test_token(
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<(), StatusCode> {
    get_uuid_from_token(&jwt_key_provider, &header_map)
        .await
        .map(|_| Ok(()))
        .map_err(|e| e.0)?
}

pub async fn route_signup(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    Json(user): Json<SignupInfo>,
) -> Result<Json<String>, (StatusCode, String)> {
    let create_user_res = store
        .create_user(user.username.into(), user.password, user.email)
        .await;

    match create_user_res {
        Ok(id) => {
            use jwt_simple::prelude::*;
            let mut claims = Claims::create(Duration::from_days(1));
            claims.subject = Some(id.to_string());

            let token = jwt_key_provider.key_pair.sign(claims);
            match token {
                Ok(token) => Ok(Json(token)),
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to sign token".to_string(),
                )),
            }
        }
        Err(e) => {
            if let sqlx::Error::Database(db_error) = &e {
                if let Some(constraint) = db_error.constraint() {
                    return Err((StatusCode::CONFLICT, constraint.to_string()));
                }
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ))
        }
    }
}

pub async fn route_login(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    Json(user): Json<LoginInfo>,
) -> Result<Json<String>, (StatusCode, String)> {
    use jwt_simple::prelude::*;
    let login_res = store
        .get_authenticated_user_id(user.username.into(), user.password)
        .await;

    if login_res.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", login_res.unwrap_err()),
        ));
    }
    let uuid = login_res.unwrap();
    let uuid = match uuid {
        Some(uuid) => uuid,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Incorrect Username or Password".to_string(),
            ));
        }
    };

    let mut claims = Claims::create(Duration::from_days(1));
    claims.subject = Some(uuid.to_string());
    let token = jwt_key_provider.key_pair.sign(claims);
    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to sign token".to_string(),
        )),
    }
}

pub async fn reset_password(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    Json(passwords): Json<NewPassword>,
) {
}
