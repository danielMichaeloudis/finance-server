use crate::{
    models::{CachedData, EncryptedDataReturn, Goal, Transaction, VendorData},
    utils::encrypt_data,
};

use super::{encryption::decrypt_data, internal_server_error, store::Store};
use axum::http::StatusCode;
use chrono::{NaiveDate, Utc};
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn process_transaction_list(
    user_or_family: &str,
    uuid: &Uuid,
    encrypted_transactions: &Vec<EncryptedDataReturn>,
    store: &Store,
) -> Result<Vec<Transaction>, (StatusCode, String)> {
    let mut decrypted_transactions = vec![];
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
                println!("Invalid transaction uuid: {:?}", transaction.uuid);
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
        decrypted_transactions.push(decrypted_transaction);
    }
    Ok(decrypted_transactions)
}

pub async fn process_cache_list(
    user_or_family: &str,
    uuid: &Uuid,
    encrypted_data: &Vec<EncryptedDataReturn>,
    store: &Store,
) -> Result<Vec<CachedData>, (StatusCode, String)> {
    let mut decrypted_cache = vec![];
    for cached_value in encrypted_data {
        let decrypted_value = decrypt_data(
            user_or_family,
            uuid,
            cached_value.encrypted_data.clone(),
            store,
        )
        .await
        .map_err(internal_server_error)?["cache"]
            .take();
        let decrypted_cached_data: Option<CachedData> = serde_json::from_value(decrypted_value)
            .unwrap_or_else(|_| {
                println!("Invalid cache uuid: {:?}", cached_value.uuid);
                None
            });
        let mut decrypted_cached_data = match decrypted_cached_data {
            None => continue,
            Some(v) => v,
        };
        decrypted_cached_data.uuid = cached_value.uuid;
        decrypted_cache.push(decrypted_cached_data);
    }
    Ok(decrypted_cache)
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
        decrypted_goal.uuid = goal.uuid;
        decrypted_goals.push(decrypted_goal)
    }
    Ok(decrypted_goals)
}

pub async fn get_all_transactions(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<Vec<Transaction>, (StatusCode, String)> {
    let family_uuid = store
        .get_family_uuid(user_uuid)
        .await
        .map_err(internal_server_error)?;

    let user_data = get_user_transactions(store, user_uuid).await?;

    if family_uuid.is_empty() {
        return Ok(user_data);
    }
    let mut fam_data = get_family_transactions(store, &family_uuid[0]).await?; //TODO: allow multiple families
    let mut data = user_data;
    data.append(&mut fam_data);
    Ok(data)
}

pub async fn get_user_transactions(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<Vec<Transaction>, (StatusCode, String)> {
    let encrypted_user_data = store
        .get_user_data(user_uuid)
        .await
        .map_err(internal_server_error)?;
    let user_data =
        process_transaction_list("user", user_uuid, &encrypted_user_data, store).await?;
    Ok(user_data)
}

pub async fn get_family_transactions(
    store: &Store,
    family_uuid: &Uuid,
) -> Result<Vec<Transaction>, (StatusCode, String)> {
    let encrypted_fam_data = store
        .get_family_data(family_uuid)
        .await
        .map_err(internal_server_error)?;
    let fam_data =
        process_transaction_list("family", family_uuid, &encrypted_fam_data, store).await?;
    Ok(fam_data)
}

pub async fn get_user_cache(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<Vec<CachedData>, (StatusCode, String)> {
    let encrypted_data = store
        .get_user_data(user_uuid)
        .await
        .map_err(internal_server_error)?;
    let data = process_cache_list("user", user_uuid, &encrypted_data, store).await?;
    Ok(data)
}

pub async fn get_family_cache(
    store: &Store,
    family_uuid: &Uuid,
) -> Result<Vec<CachedData>, (StatusCode, String)> {
    let encrypted_data = store
        .get_family_data(family_uuid)
        .await
        .map_err(internal_server_error)?;
    let data = process_cache_list("family", family_uuid, &encrypted_data, store).await?;
    Ok(data)
}

pub async fn encrypt_and_add_transaction(
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
    if user_or_family == "family" {
        store
            .add_family_data(&uuid, transaction_data)
            .await
            .map_err(internal_server_error)?;
    } else {
        store
            .add_user_data(&uuid, transaction_data)
            .await
            .map_err(internal_server_error)?;
    }
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
    let mut family_transactions = vec![];

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
            user_uuid.to_owned()
        } else {
            match family_uuid.is_none() {
                false => *family_uuid.unwrap(),
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
        if user_or_family == "family" {
            family_transactions.push(transaction_data);
        } else {
            user_transactions.push(transaction_data);
        }
    }
    let mut errors = vec![];
    //TODO need to make more safe if errors occur
    for transaction in user_transactions {
        let _ = store
            .add_user_data(user_uuid, transaction)
            .await
            .map_err(|e| errors.push(e.to_string()));
    }
    for transaction in family_transactions {
        let _ = store
            .add_family_data(
                family_uuid.expect("No transactions should be pushed here if this is none"),
                transaction,
            )
            .await
            .map_err(|e| errors.push(e.to_string()));
    }
    Ok(errors)
}

pub async fn add_cached_value(
    user_or_family: &str,
    uuid: &Uuid,
    name: String,
    value: Value,
    store: &Store,
) -> Result<(), (StatusCode, String)> {
    let time = Utc::now().naive_utc();
    let data = json!({"cache": {"name": name, "value": value, "created": time}});
    println!("added cache value: {}", data);
    let encrypted_value = encrypt_data(user_or_family, uuid, data, store)
        .await
        .map_err(internal_server_error)?;
    if user_or_family == "family" {
        store
            .add_family_data(uuid, encrypted_value)
            .await
            .map_err(internal_server_error)?;
    } else {
        store
            .add_user_data(uuid, encrypted_value)
            .await
            .map_err(internal_server_error)?;
    }
    Ok(())
}

pub async fn process_vendor_data(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<Vec<VendorData>, (StatusCode, String)> {
    let transactions = get_all_transactions(store, user_uuid).await?;
    let mut vendor_data: Vec<VendorData> = vec![];
    for transaction in transactions {
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

pub async fn get_total_in_out(
    store: &Store,
    user_uuid: &Uuid,
) -> Result<Value, (StatusCode, String)> {
    let transactions = get_all_transactions(store, user_uuid).await?;
    let username = &store
        .get_username(user_uuid)
        .await
        .map_err(internal_server_error)?;
    let incomming = transactions.iter().fold(0.0, |acc, t| {
        if t.cost > 0.0 && t.buyer.to_lowercase() == username.to_lowercase() {
            acc + t.cost
        } else {
            acc
        }
    });
    let outgoing = transactions.iter().fold(0.0, |acc, t| {
        if t.cost < 0.0 && t.buyer.to_lowercase() == username.to_lowercase() {
            acc - t.cost
        } else {
            acc
        }
    });
    let total = incomming - outgoing;
    let res = json!({"total": total, "incomming": incomming, "outgoing": outgoing});
    Ok(res)
}

pub async fn get_total_spent(store: &Store, user_uuid: &Uuid) -> Result<f64, (StatusCode, String)> {
    let family_uuid = store
        .get_family_uuid(user_uuid)
        .await
        .map_err(internal_server_error)?;
    let user_sum = get_user_spent(store, user_uuid).await?;
    if family_uuid.is_empty() {
        return Ok(user_sum);
    }
    Ok(user_sum + get_family_spent(store, &family_uuid[0]).await?)
}

pub async fn get_user_spent(store: &Store, user_uuid: &Uuid) -> Result<f64, (StatusCode, String)> {
    let cache = get_user_cache(store, user_uuid).await?;
    let mut cached_total: f64 = 0.0;
    let mut cached_timestamp = NaiveDate::MIN;
    let mut cached_uuid: Option<Uuid> = None;
    for cached_value in cache {
        if cached_value.name == "total" {
            cached_total = match serde_json::from_value(cached_value.value) {
                Ok(v) => v,
                Err(_) => {
                    continue;
                }
            };
            cached_timestamp = cached_value.created;
            cached_uuid = cached_value.uuid;
            break;
        }
    }
    println!("cached total: {}", cached_total);
    let transactions = get_user_transactions(store, user_uuid).await?;
    let transactions = transactions
        .into_iter()
        .filter(|t| t.date.unwrap_or(NaiveDate::MAX) > cached_timestamp)
        .collect::<Vec<_>>();

    let sum = transactions
        .iter()
        .fold(cached_total, |acc, t| acc + t.cost);
    if cached_total != sum {
        add_cached_value("user", user_uuid, "total".to_string(), json!(sum), store).await?;
        if cached_uuid.is_some() && cached_total != sum {
            println!("removing cached data uuid: {}", cached_uuid.unwrap());
            store
                .remove_user_data(user_uuid, &cached_uuid.unwrap())
                .await
                .map_err(internal_server_error)?;
        }
    }
    Ok(sum)
}

pub async fn get_family_spent(
    store: &Store,
    family_uuid: &Uuid,
) -> Result<f64, (StatusCode, String)> {
    let cache = get_family_cache(store, family_uuid).await?;
    let mut cached_total: f64 = 0.0;
    let mut cached_timestamp = NaiveDate::MIN;
    let mut cached_uuid: Option<Uuid> = None;
    for cached_value in cache {
        if cached_value.name == "total" {
            cached_total = match serde_json::from_value(cached_value.value) {
                Ok(v) => v,
                Err(_) => {
                    continue;
                }
            };
            cached_timestamp = cached_value.created;
            cached_uuid = cached_value.uuid;
            break;
        }
    }
    let transactions = get_family_transactions(store, family_uuid).await?;
    let transactions = transactions
        .into_iter()
        .filter(|t| t.date.unwrap_or(NaiveDate::MAX) > cached_timestamp)
        .collect::<Vec<_>>();

    let sum = transactions
        .iter()
        .fold(cached_total, |acc, t| acc + t.cost);
    if cached_total != sum {
        add_cached_value(
            "family",
            family_uuid,
            "total".to_string(),
            json!(sum),
            store,
        )
        .await?;
        if cached_uuid.is_some() {
            store
                .remove_family_data(family_uuid, &cached_uuid.unwrap())
                .await
                .map_err(internal_server_error)?;
        }
    }
    Ok(sum)
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
