use axum::http::StatusCode;
use chrono::Utc;
use sqlx::{postgres::PgPoolOptions, query, query_as, query_scalar, types::Uuid, PgPool};

use crate::models::EncryptedDataReturn;

use super::{
    encryption::{decrypt_string, get_new_encrypted_key},
    internal_server_error,
};

pub async fn get_store() -> Result<Store, sqlx::Error> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(Store::new(PgPoolOptions::new().connect(&db_url).await?))
}

#[derive(Clone)]
pub(crate) struct Store {
    pool: PgPool,
}

impl Store {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_encryption_key(&self, uuid: &Uuid) -> Result<Vec<u8>, sqlx::Error> {
        query!(
            r#"--sql
            select encryption_key from users where user_uuid = $1
        "#,
            uuid
        )
        .fetch_one(&self.pool)
        .await
        .map(|user| user.encryption_key)
    }

    pub async fn get_family_encryption_key(&self, uuid: &Uuid) -> Result<Vec<u8>, sqlx::Error> {
        query!(
            r#"--sql
            select encryption_key from families where family_uuid = $1
        "#,
            uuid
        )
        .fetch_one(&self.pool)
        .await
        .map(|fam| fam.encryption_key)
    }

    pub async fn get_username(&self, user_uuid: &Uuid) -> Result<String, sqlx::Error> {
        let res = query!(
            r#"--sql
            select username from users where user_uuid = $1
        "#,
            user_uuid
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(res.username)
    }

    pub async fn get_user_data(
        &self,
        user_uuid: &Uuid,
    ) -> Result<Vec<EncryptedDataReturn>, sqlx::Error> {
        query_as!(
            EncryptedDataReturn,
            r#"--sql
            select uuid, user_uuid "owner_uuid", encrypted_data, data_time from public.user_data where user_uuid = $1
        "#, user_uuid
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn add_user_data(
        &self,
        user_uuid: &Uuid,
        transaction_data: Vec<u8>,
    ) -> Result<Uuid, sqlx::Error> {
        let time = Utc::now().naive_utc();
        let res = query_scalar!(
            r#"--sql
                insert into user_data (user_uuid, encrypted_data, data_time) 
                values ($1, $2, $3)
                returning user_uuid
            
        "#,
            user_uuid,
            transaction_data,
            time
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(res)
    }

    pub async fn edit_user_data(
        &self,
        user_uuid: &Uuid,
        data_uuid: &Uuid,
        data: Vec<u8>,
    ) -> Result<Uuid, sqlx::Error> {
        let time = Utc::now().naive_utc();
        let mut tx = self.pool.begin().await?;
        println!("Deleting transaction");
        query!("delete from user_data where uuid=$1", data_uuid)
            .execute(&mut *tx)
            .await?;
        println!("Inserting new transaction");
        let res = query_scalar!(
            r#"--sql
                insert into user_data (user_uuid, encrypted_data, data_time) 
                values ($1, $2, $3)
                returning uuid;
            
        "#,
            user_uuid,
            data,
            time
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        println!("Committed. Uuid: {res:?}");

        Ok(res)
    }

    pub async fn remove_user_data(
        &self,
        user_uuid: &Uuid,
        data_uuid: &Uuid,
    ) -> Result<(), sqlx::Error> {
        let _res = query!(
            r#"--sql
        delete from user_data where
        uuid = $1 and user_uuid = $2
        "#,
            data_uuid,
            user_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_user(
        &self,
        username: ValidUsername,
        password: String,
        email: String,
    ) -> Result<Uuid, sqlx::Error> {
        let password_hash = password_auth::generate_hash(password);
        let key = get_new_encrypted_key(None).await.unwrap();
        let user = query!(
            r#"--sql
            insert into users (username, password_hash, email, encryption_key)
            values ($1, $2, $3, $4)
            returning user_uuid 
        "#,
            username.0,
            password_hash,
            email,
            &key
        )
        .fetch_one(&self.pool)
        .await;
        match user {
            Ok(user) => Ok(user.user_uuid),
            Err(e) => Err(e),
        }
    }
    pub async fn get_authenticated_user_id(
        &self,
        username: ValidUsername,
        password: String,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let user = query!(
            r#"--sql
            select user_uuid, password_hash 
            from users 
            where username = $1
        "#,
            username.0
        )
        .fetch_optional(&self.pool)
        .await?;
        if let Some(user) = user {
            if password_auth::verify_password(password, &user.password_hash).is_ok() {
                return Ok(Some(user.user_uuid));
            }
        }
        Ok(None)
    }

    pub async fn get_family_uuid(&self, uuid: &Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let families = query!(
            r#"--sql
        select family_uuid from link_users_families where user_uuid = $1"#,
            uuid,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(families.iter().map(|v| v.family_uuid).collect())
    }

    pub async fn get_family_from_join_code(
        &self,
        join_code: &str,
    ) -> Result<(Uuid, String), (StatusCode, String)> {
        let prefix = match join_code.get(0..4) {
            Some(p) => p,
            None => return Err((StatusCode::UNAUTHORIZED, "Invalid join code".to_string())),
        };
        let res = query!(
            r#"--sql
        select family_uuid, join_code from families where join_code_prefix = $1;"#,
            prefix
        )
        .fetch_one(&self.pool)
        .await
        .map_err(internal_server_error)?;

        let join_code = decrypt_string("family", &res.family_uuid, res.join_code, self)
            .await
            .map_err(internal_server_error)?;

        Ok((res.family_uuid, join_code))
    }

    pub async fn add_user_to_family(
        &self,
        user_uuid: &Uuid,
        family_uuid: &Uuid,
    ) -> Result<(), sqlx::Error> {
        let _res = query!(
            r#"--sql
            insert into link_users_families (user_uuid, family_uuid) values ($1, $2)
        "#,
            user_uuid,
            family_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_family(
        &self,
        join_code_prefix: String,
        join_code_encrypted: Vec<u8>,
        encryption_key: Vec<u8>,
    ) -> Result<Uuid, String> {
        let res = query!(
            r#"--sql
            insert into families (join_code_prefix, join_code, encryption_key) 
            values ($1, $2, $3)
            returning family_uuid
        "#,
            join_code_prefix,
            join_code_encrypted,
            encryption_key
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        let fam_uuid = res.family_uuid;
        Ok(fam_uuid)
    }

    pub async fn get_family_join_code_from_user_uuid(
        &self,
        user_uuid: &Uuid,
    ) -> Result<Option<String>, String> {
        let res = query!(
            r#"--sql
                SELECT f.join_code, f.family_uuid
                FROM families f
                WHERE f.family_uuid = (
                    SELECT family_uuid
                    FROM link_users_families
                    WHERE user_uuid = $1
                    LIMIT 1
        );"#,
            user_uuid
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        let (encrypted_join_code, family_uuid) = match res {
            Some(res) => (res.join_code, res.family_uuid),
            None => {
                return Ok(None);
            }
        };
        let join_code = decrypt_string("family", &family_uuid, encrypted_join_code, self).await?;
        Ok(Some(join_code))
    }
}

pub(crate) struct ValidUsername(String);

impl From<String> for ValidUsername {
    fn from(username: String) -> Self {
        let cleaned_username = username
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        Self(cleaned_username)
    }
}
