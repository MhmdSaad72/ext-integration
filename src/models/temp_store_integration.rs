use actix_web::web::Data;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use log::error;
use serde_json::Value;

use crate::{
    errors::app_error::AppError,
    models::{integration_platform::IntegrationPlatform, user_model::NewUser},
    schema::temp_stores_integrations,
    utilities::database::get_db_connection,
    DbPool,
};

use super::user_model::User;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = temp_stores_integrations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct TempStoreIntegration {
    pub id: i64,
    pub store_name: String,
    pub store_url: String,
    pub email: String,
    pub shop_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub expires: i64,
    pub default_carrier_id: i32,
    pub integration_platform_id: i64,
    pub odd_enabled: bool,
    pub new_store_id: Option<i64>,
    pub new_assigned_user_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub auth_code: Option<String>,
    pub authorization_code: Option<String>,
}

#[derive(Insertable, Debug, Default)]
#[diesel(table_name = temp_stores_integrations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTempStoreIntegration {
    pub shop_id: i32,
    pub integration_platform_id: i64,
    pub store_name: String,
    pub store_url: String,
    pub email: String,
    pub auth_code: Option<String>,
    pub default_carrier_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub authorization_code: Option<String>,
    pub expires: i64,
    pub odd_enabled: bool,
}

impl TempStoreIntegration {
    pub async fn update_or_create(
        info: Value,
        payload: Value,
        db_pool: &Data<DbPool>,
    ) -> Result<Self, AppError> {
        let connection = &mut get_db_connection(db_pool)?;

        let _user = find_or_create_user(&info, db_pool).await?;

        let platform_id =
            IntegrationPlatform::find_by_name("salla_plugin".to_string(), db_pool).await?;
        let new_data = parse_store_data(info, payload, platform_id.id);
        insert_or_update_store(new_data, connection)
    }
}

async fn find_or_create_user(info: &Value, db_pool: &Data<DbPool>) -> Result<User, AppError> {
    let email = info["data"]["email"].as_str().unwrap_or("");
    match User::find_by_email(email, db_pool).await {
        Ok(user) => Ok(user),
        Err(_) => {
            let new_user = NewUser {
                first_name: info["data"]["name"].as_str().unwrap_or("").to_string(),
                last_name: info["data"]["name"].as_str().unwrap_or("").to_string(),
                email: email.to_string(),
                company_name: info["data"]["name"].as_str().unwrap_or("").to_string(),
                store_url: info["data"]["domain"].as_str().unwrap_or("").to_string(),
                birth_day: Utc::now(),
                email_verified_at: Utc::now(),
                affiliation_code: format!("{}-{}", email, Utc::now().timestamp()),
                ..Default::default()
            };
            User::create_user(new_user, db_pool).await
        }
    }
}

fn parse_store_data(info: Value, payload: Value, platform_id: i64) -> NewTempStoreIntegration {
    NewTempStoreIntegration {
        shop_id: info["data"]["id"].as_i64().unwrap_or(0) as i32,
        integration_platform_id: platform_id,
        store_name: info["data"]["name"].as_str().unwrap_or("").to_string(),
        store_url: info["data"]["domain"].as_str().unwrap_or("").to_string(),
        email: info["data"]["email"].as_str().unwrap_or("").to_string(),
        auth_code: Some(
            format!(
                "{}-{}",
                info["data"]["id"].as_i64().unwrap_or(0),
                Utc::now().timestamp()
            )
            .to_string(),
        ),
        access_token: payload["data"]["access_token"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        refresh_token: payload["data"]["refresh_token"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        authorization_code: None,
        expires: payload["data"]["expires"].as_i64().unwrap_or(0),
        default_carrier_id: 2,
        ..Default::default()
    }
}

fn insert_or_update_store(
    new_data: NewTempStoreIntegration,
    connection: &mut PgConnection,
) -> Result<TempStoreIntegration, AppError> {
    use self::temp_stores_integrations::dsl::*;

    let temp_store = temp_stores_integrations
        .filter(shop_id.eq(new_data.shop_id))
        .filter(integration_platform_id.eq(new_data.integration_platform_id))
        .first::<TempStoreIntegration>(connection);

    match temp_store {
        Ok(store) => {
            diesel::update(temp_stores_integrations.find(store.id))
                .set((
                    store_name.eq(new_data.store_name),
                    store_url.eq(new_data.store_url),
                    email.eq(new_data.email),
                    auth_code.eq(new_data.auth_code),
                    default_carrier_id.eq(new_data.default_carrier_id),
                    access_token.eq(new_data.access_token),
                    refresh_token.eq(new_data.refresh_token),
                    authorization_code.eq(new_data.authorization_code),
                    expires.eq(new_data.expires),
                ))
                .execute(connection)
                .map_err(|e| {
                    error!(target: "salla_plugin", "Error updating store: {:?}", e);
                    AppError::DatabaseError {
                        field: "temp_store_integration".into(),
                        source: e,
                    }
                })?;

            return Ok(store);
        }
        Err(_e) => diesel::insert_into(temp_stores_integrations)
            .values(&new_data)
            .get_result::<TempStoreIntegration>(connection)
            .map_err(|e| {
                error!(target: "salla_plugin", "Error inserting store: {:?}", e);
                AppError::DatabaseError {
                    field: "temp_store_integration".into(),
                    source: e,
                }
            }),
    }
}
