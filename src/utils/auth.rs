use axum::http::{HeaderMap, StatusCode};
use jwt_simple::prelude::{ECDSAP256PublicKeyLike, NoCustomClaims};
use uuid::Uuid;

use super::jwt::JWTKeyProvider;

pub async fn get_uuid_from_token(
    jwt_key_provider: &JWTKeyProvider,
    header_map: &HeaderMap,
) -> Result<Uuid, (StatusCode, String)> {
    match header_map.get("authorization") {
        Some(auth_header) => {
            if let Ok(auth_header_str) = auth_header.to_str() {
                if !auth_header_str.starts_with("Bearer ") {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        "Invalid Auth Headers. No Bearer".to_string(),
                    ));
                }
                let token = auth_header_str.trim_start_matches("Bearer ");
                let claims_res = jwt_key_provider
                    .key_pair
                    .public_key()
                    .verify_token::<NoCustomClaims>(token, None);

                match claims_res {
                    Ok(claims) => {
                        let uuid: Uuid = claims.subject.unwrap().parse().unwrap();
                        return Ok(uuid);
                    }
                    Err(e) => {
                        return Err((StatusCode::UNAUTHORIZED, format!("Invalid Token: {}", e)))
                    }
                }
            }
            Err((StatusCode::UNAUTHORIZED, "Invalid Auth Headers".to_string()))
        }
        None => Err((StatusCode::UNAUTHORIZED, "No Auth Headers".to_string())),
    }
}

//TODO: get from request parts
