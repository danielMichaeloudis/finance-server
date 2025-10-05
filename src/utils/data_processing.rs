use std::collections::HashMap;

use crate::{
    models::{EncryptedDataReturn, Goal, Transaction, VendorData},
    utils::encrypt_data,
};

use super::{encryption::decrypt_data, internal_server_error, store::Store};
use axum::http::StatusCode;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

pub async fn process_transaction_list(
    user_or_family: &str,
    uuid: &Uuid,
    encrypted_transactions: &Vec<EncryptedDataReturn>,
    store: &Store,
) -> Result<HashMap<Uuid, Transaction>, (StatusCode, String)> {
    let mut decrypted_transactions = HashMap::new();
    for transaction in encrypted_transactions {
        let decrypted_value = decrypt_data(
            user_or_family,
            uuid,
            transaction.encrypted_data.clone(),
            store,
        )
        .await
        .map_err(internal_server_error)?["transaction"]
            .take();
        let decrypted_transaction: Option<Transaction> = serde_json::from_value(decrypted_value)
            .unwrap_or_else(|_| {
                println!("Invalid transaction. Uuid: {:?}", transaction.uuid);
                None
            });
        let mut decrypted_transaction = match decrypted_transaction {
            None => continue,
            Some(v) => v,
        };
        decrypted_transaction.date = match decrypted_transaction.date {
            Some(t) => Some(t),
            None => transaction.data_time.map(|d| d.date()),
        };
        decrypted_transactions.insert(transaction.uuid, decrypted_transaction);
    }
    Ok(decrypted_transactions)
}

pub async fn process_goals_list(
    user_uuid: &Uuid,
    encrypted_data: &Vec<EncryptedDataReturn>,
    store: &Store,
) -> Result<Vec<Goal>, (StatusCode, String)> {
    let mut decrypted_goals = vec![];
    for goal in encrypted_data {
        let decryped_value = decrypt_data("user", user_uuid, goal.encrypted_data.clone(), store)
            .await
            .map_err(internal_server_error)?["goal"]
            .take();
        let decrypted_goal: Option<Goal> =
            serde_json::from_value(decryped_value).unwrap_or_else(|_| {
                println!("Invalid cache uuid: {:?}", goal.uuid);
                None
            });
        let mut decrypted_goal = match decrypted_goal {
            None => continue,
            Some(v) => v,
        };
        decrypted_goal.uuid = Some(goal.uuid);
        decrypted_goals.push(decrypted_goal)
    }
    Ok(decrypted_goals)
}

pub async fn get_all_transactions(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<HashMap<Uuid, Transaction>, (StatusCode, String)> {
    get_user_transactions(store, user_uuid).await
}

pub async fn get_user_transactions(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<HashMap<Uuid, Transaction>, (StatusCode, String)> {
    let encrypted_user_data = store
        .get_user_data(user_uuid)
        .await
        .map_err(internal_server_error)?;
    let user_data =
        process_transaction_list("user", user_uuid, &encrypted_user_data, store).await?;
    Ok(user_data)
}

pub async fn encrypt_add_transaction(
    store: &Store,
    user_uuid: &Uuid,
    transaction: Transaction,
) -> Result<(), (StatusCode, String)> {
    let mut user_or_family = match transaction.input_for_family {
        Some(fam) => {
            if fam {
                "family"
            } else {
                "user"
            }
        }
        None => "user",
    };
    let uuid = if user_or_family != "family" {
        user_uuid.to_owned()
    } else {
        let uuid_res = store
            .get_family_uuid(user_uuid)
            .await
            .map_err(internal_server_error)?;
        match uuid_res.is_empty() {
            false => uuid_res[0],
            true => {
                user_or_family = "user";
                user_uuid.to_owned()
            }
        }
    };
    let mut data = json!({"transaction": transaction});
    let _ = data["transaction"]["input_for_family"].take(); //not needed after adding
    let transaction_data = encrypt_data(user_or_family, &uuid, data, store)
        .await
        .map_err(internal_server_error)?;
    store
        .add_user_data(&uuid, transaction_data)
        .await
        .map_err(internal_server_error)?;

    Ok(())
}

pub async fn encrypt_add_transactions(
    store: &Store,
    user_uuid: &Uuid,
    transactions: Vec<Transaction>,
) -> Result<Vec<String>, (StatusCode, String)> {
    let uuid_res = store
        .get_family_uuid(user_uuid)
        .await
        .map_err(internal_server_error)?;
    let family_uuid = uuid_res.first();

    let mut user_transactions = vec![];

    for transaction in transactions {
        let mut user_or_family = match transaction.input_for_family {
            Some(fam) => {
                if fam {
                    "family"
                } else {
                    "user"
                }
            }
            None => "user",
        };
        let uuid = if user_or_family != "family" {
            user_uuid
        } else {
            match family_uuid {
                Some(u) => u,
                None => {
                    user_or_family = "user";
                    user_uuid
                }
            }
        };
        let mut data = json!({"transaction": transaction});
        let _ = data["transaction"]["input_for_family"].take(); //not needed after adding
        let transaction_data = encrypt_data(user_or_family, uuid, data, store)
            .await
            .map_err(internal_server_error)?;
        user_transactions.push(transaction_data);
    }
    let mut errors = vec![];
    //TODO need to make more safe if errors occur
    for transaction in user_transactions {
        let _ = store
            .add_user_data(user_uuid, transaction)
            .await
            .map_err(|e| errors.push(e.to_string()));
    }
    Ok(errors)
}

pub async fn encrypt_edit_transaction(
    store: &Store,
    user_uuid: &Uuid,
    edited: Transaction,
) -> Result<(), (StatusCode, String)> {
    let data_uuid = edited.uuid.unwrap();

    let mut data = json!({"transaction": edited});
    let _ = data["transaction"]["input_for_family"].take(); //not needed after adding
    let _ = data["transaction"]["uuid"].take();
    let encrypted_data = encrypt_data("user", user_uuid, data, store)
        .await
        .map_err(internal_server_error)?;
    store
        .edit_user_data(user_uuid, &data_uuid, encrypted_data)
        .await
        .map_err(internal_server_error)?;
    Ok(())
}

pub async fn remove_transaction(
    store: &Store,
    user_uuid: &Uuid,
    uuid: &Uuid,
) -> Result<(), (StatusCode, String)> {
    store
        .remove_user_data(user_uuid, uuid)
        .await
        .map_err(internal_server_error)?;
    Ok(())
}

pub async fn process_vendor_data(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<Vec<VendorData>, (StatusCode, String)> {
    let transactions = get_all_transactions(store, user_uuid).await?;
    let mut vendor_data: Vec<VendorData> = vec![];
    for (_, transaction) in transactions {
        let pos = vendor_data
            .iter()
            .position(|v| v.vendor == transaction.vendor);
        match pos {
            Some(p) => {
                vendor_data[p].amount_spent += transaction.cost;
                vendor_data[p].times_bought_from += 1;
            }
            None => {
                vendor_data.push(VendorData {
                    vendor: transaction.vendor,
                    times_bought_from: 1,
                    amount_spent: transaction.cost,
                }); //add stats for people
            }
        }
    }
    Ok(vendor_data)
}

pub async fn get_goals(store: &Store, user_uuid: &Uuid) -> Result<Vec<Goal>, (StatusCode, String)> {
    let user_data = store
        .get_user_data(user_uuid)
        .await
        .map_err(internal_server_error)?;
    let goals = process_goals_list(user_uuid, &user_data, store).await?;
    println!("{:?}", goals);
    Ok(goals)
}

pub async fn set_goal(
    store: &Store,
    user_uuid: &Uuid,
    goal: Goal,
) -> Result<(), (StatusCode, String)> {
    let goals = get_goals(store, user_uuid).await?;
    let adding_goals: Vec<&Goal> = goals
        .iter()
        .filter(|g| g.name == goal.name && g.uuid.is_some())
        .collect();
    for g in adding_goals {
        store
            .remove_user_data(user_uuid, &g.uuid.unwrap())
            .await
            .map_err(internal_server_error)?
    }
    let mut goal = goal;
    goal.created = Some(Utc::now().naive_utc());
    let to_encrypt = json!({"goal": goal});
    let encrypted_goal = encrypt_data("user", user_uuid, to_encrypt, store)
        .await
        .map_err(internal_server_error)?;
    store
        .add_user_data(user_uuid, encrypted_goal)
        .await
        .map_err(internal_server_error)?;
    Ok(())
}
