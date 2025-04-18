use crate::{
    errors::app_error::AppError,
    schema::users::{self},
    utilities::database::get_db_connection,
    DbPool,
};
use actix_web::web::Data;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use log::error;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub email: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub password: String,
    pub expiry_date: Option<DateTime<Utc>>,
    pub credit: f64,
    pub company_name: String,
    pub attachment_url: Option<String>,
    pub birth_day: Option<DateTime<Utc>>,
    pub mobile_number: String,
    pub approved: bool,
    pub bank_is_activated: bool,
    pub beneficiary_name: Option<String>,
    pub beneficiary_address_building_no: Option<String>,
    pub beneficiary_address_street_name: Option<String>,
    pub beneficiary_address_neighborhood: Option<String>,
    pub beneficiary_address_city: Option<String>,
    pub bank_name: Option<String>,
    pub account_number: Option<String>,
    pub iban: Option<String>,
    pub is_admin: Option<bool>,
    pub plan_id: Option<i64>,
    pub remember_token: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_name: String,
    pub last_subscription_date: Option<DateTime<Utc>>,
    pub last_subscription_fees: Option<f64>,
    pub phone_valid: bool,
    pub avg_of_monthly_shipments: Option<i64>,
    pub shipment_weights_min: Option<f64>,
    pub shipment_weights_max: Option<f64>,
    pub store_url: Option<String>,
    pub phone_verified: bool,
    pub monthly_duration_start: Option<DateTime<Utc>>,
    pub monthly_duration_end: Option<DateTime<Utc>>,
    pub phone_last_update: Option<NaiveDate>,
    pub active: bool,
    pub role_id: Option<i64>,
    pub affiliation_code: String,
    pub affiliation_active: bool,
    pub affiliator_id: Option<i64>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub auto_created: bool,
    pub info_completed: bool,
    pub post_paid: bool,
    pub after_dlv_cod_diff: bool,
    pub send_wa_msg_for_returns: bool,
    pub test_mode: bool,
    pub account_id: Option<i64>,
    pub bonus_credit_on_charge: bool,
    pub thirdmile_agent: bool,
}

#[derive(Insertable, Debug, Default)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub company_name: String,
    pub birth_day: DateTime<Utc>,
    pub mobile_number: String,
    pub attachment_url: String,
    pub store_url: String,
    pub avg_of_monthly_shipments: i64,
    pub shipment_weights_min: f64,
    pub shipment_weights_max: f64,
    pub phone_valid: bool,
    pub phone_verified: bool,
    pub auto_created: bool,
    pub info_completed: bool,
    pub email_verified_at: DateTime<Utc>,
    pub affiliation_code: String,
}

impl User {
    pub async fn find_by_email(_email: &str, conn: &Data<DbPool>) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;
        let connection = &mut get_db_connection(conn)?;
        users
            .filter(email.eq(_email))
            .select(User::as_select())
            .first::<User>(connection)
            .map_err(|_| AppError::DatabaseError {
                field: "email".into(),
                source: diesel::result::Error::NotFound,
            })
    }

    pub async fn create_user(new_user: NewUser, db_pool: &Data<DbPool>) -> Result<Self, AppError> {
        use crate::schema::users::dsl::*;
        let connection = &mut get_db_connection(db_pool)?;

        let result = diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result::<User>(connection);

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                error!(target:"salla_plugin","Error creating user: {:?}", e);
                Err(AppError::DatabaseError {
                    field: "user".into(),
                    source: e,
                })
            }
        }

        // Ok(result?) // Return the created user

        // result
    }
}
