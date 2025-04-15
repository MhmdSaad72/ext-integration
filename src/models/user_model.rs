use crate::{
    errors::app_error::AppError, schema::users, utilities::database::get_db_connection, DbPool,
};
use actix_web::web::Data;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    id: i64,
    first_name: String,
    email: String,
    email_verified_at: Option<DateTime<Utc>>,
    password: String,
    expiry_date: Option<DateTime<Utc>>,
    credit: f64,
    company_name: String,
    attachment_url: Option<String>,
    birth_day: Option<DateTime<Utc>>,
    mobile_number: String,
    approved: bool,
    bank_is_activated: bool,
    beneficiary_name: Option<String>,
    beneficiary_address_building_no: Option<String>,
    beneficiary_address_street_name: Option<String>,
    beneficiary_address_neighborhood: Option<String>,
    beneficiary_address_city: Option<String>,
    bank_name: Option<String>,
    account_number: Option<String>,
    iban: Option<String>,
    is_admin: Option<bool>,
    plan_id: Option<i64>,
    remember_token: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    last_name: String,
    last_subscription_date: Option<DateTime<Utc>>,
    last_subscription_fees: Option<f64>,
    phone_valid: bool,
    avg_of_monthly_shipments: Option<i64>,
    shipment_weights_min: Option<f64>,
    shipment_weights_max: Option<f64>,
    store_url: Option<String>,
    phone_verified: bool,
    monthly_duration_start: Option<DateTime<Utc>>,
    monthly_duration_end: Option<DateTime<Utc>>,
    phone_last_update: Option<DateTime<Utc>>,
    active: bool,
    role_id: Option<i64>,
    affiliation_code: String,
    affiliation_active: bool,
    affiliator_id: Option<i64>,
    deleted_at: Option<DateTime<Utc>>,
    auto_created: bool,
    info_completed: bool,
    post_paid: bool,
    after_dlv_cod_diff: bool,
    send_wa_msg_for_returns: bool,
    test_mode: bool,
    account_id: Option<i64>,
    bonus_credit_on_charge: bool,
    thirdmile_agent: bool,
}

impl User {
    pub fn find_by_email(email: &str, conn: &Data<DbPool>) -> Result<Self, AppError> {
        use crate::schema::users::dsl::*;
        let connection = &mut get_db_connection(conn)?;
        users
            .filter(email.eq(email))
            .first::<User>(connection)
            .map_err(|_| AppError::DatabaseError {
                field: "email".into(),
                source: diesel::result::Error::NotFound,
            })
    }
}
