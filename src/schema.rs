// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "availability_status"))]
    pub struct AvailabilityStatus;
}

diesel::table! {
    cache (key) {
        #[max_length = 255]
        key -> Varchar,
        value -> Text,
        expiration -> Int4,
    }
}

diesel::table! {
    cache_locks (key) {
        #[max_length = 255]
        key -> Varchar,
        #[max_length = 255]
        owner -> Varchar,
        expiration -> Int4,
    }
}

diesel::table! {
    carriers (id) {
        id -> Int8,
        carrier -> Varchar,
        cc_price -> Float8,
        cod_price -> Float8,
        logo -> Nullable<Varchar>,
        user_label_en -> Varchar,
        user_label_ar -> Varchar,
        active -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        notes -> Nullable<Text>,
        handler_class -> Nullable<Varchar>,
        def_service_id -> Nullable<Int8>,
        request_urls -> Nullable<Json>,
    }
}

diesel::table! {
    failed_jobs (id) {
        id -> Int8,
        #[max_length = 255]
        uuid -> Varchar,
        connection -> Text,
        queue -> Text,
        payload -> Text,
        exception -> Text,
        failed_at -> Timestamp,
    }
}

diesel::table! {
    integrated_stores (id) {
        id -> Int8,
        user_id -> Int8,
        integration_platform_id -> Int8,
        is_stopped -> Bool,
        is_disabled -> Bool,
        store_url -> Varchar,
        store_name -> Varchar,
        token -> Nullable<Text>,
        refresh_token -> Nullable<Text>,
        shop_id -> Nullable<Varchar>,
        default_carrier_id -> Nullable<Int8>,
        default_shipping_address_id -> Nullable<Int8>,
        integration_info -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        integration_token_data -> Nullable<Text>,
        odd_enabled -> Bool,
        whpm_enabled -> Bool,
        webhook_authorization -> Text,
        account_id -> Nullable<Int8>,
        reqs_mapper_class -> Nullable<Varchar>,
        integration_abilities -> Json,
        order_proc_method -> Nullable<Varchar>,
        group_id -> Nullable<Int8>,
        auth_code -> Nullable<Varchar>,
        authorization_code -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AvailabilityStatus;

    integration_platforms (id) {
        id -> Int8,
        platform -> Varchar,
        label -> Nullable<Varchar>,
        gateway -> Nullable<Varchar>,
        required_fields -> Nullable<Json>,
        gateway_requirements -> Nullable<Text>,
        is_ready -> Bool,
        enabled -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        integration_abilities -> Json,
        def_store_driver -> Nullable<Varchar>,
        reqs_mapper_class -> Nullable<Varchar>,
        def_order_proc_method -> Varchar,
        order_proc_changeable -> Bool,
        img -> Nullable<Varchar>,
        public_showable -> Bool,
        video_guide_url -> Nullable<Varchar>,
        group_id -> Nullable<Int8>,
        img_light -> Nullable<Varchar>,
        img_max_height -> Nullable<Varchar>,
        availability_status -> AvailabilityStatus,
        guide_docs -> Nullable<Jsonb>,
    }
}

diesel::table! {
    job_batches (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        total_jobs -> Int4,
        pending_jobs -> Int4,
        failed_jobs -> Int4,
        failed_job_ids -> Text,
        options -> Nullable<Text>,
        cancelled_at -> Nullable<Int4>,
        created_at -> Int4,
        finished_at -> Nullable<Int4>,
    }
}

diesel::table! {
    jobs (id) {
        id -> Int8,
        #[max_length = 255]
        queue -> Varchar,
        payload -> Text,
        attempts -> Int2,
        reserved_at -> Nullable<Int4>,
        available_at -> Int4,
        created_at -> Int4,
    }
}

diesel::table! {
    migrations (id) {
        id -> Int4,
        #[max_length = 255]
        migration -> Varchar,
        batch -> Int4,
    }
}

diesel::table! {
    password_reset_tokens (email) {
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AvailabilityStatus;

    platform_groups (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        label -> Nullable<Varchar>,
        img -> Nullable<Varchar>,
        img_light -> Nullable<Varchar>,
        img_max_height -> Nullable<Varchar>,
        availability_status -> AvailabilityStatus,
        showable -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    salla_webhooks (id) {
        id -> Int8,
        salla_plugin_name -> Varchar,
        salla_plugin_id -> Int8,
        event -> Varchar,
        merchant_id -> Int8,
        order_id -> Nullable<Int8>,
        order_reference_id -> Nullable<Int8>,
        payload -> Jsonb,
        processed -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (id) {
        #[max_length = 255]
        id -> Varchar,
        user_id -> Nullable<Int8>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        payload -> Text,
        last_activity -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        email_verified_at -> Nullable<Timestamp>,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 100]
        remember_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cache,
    cache_locks,
    carriers,
    failed_jobs,
    integrated_stores,
    integration_platforms,
    job_batches,
    jobs,
    migrations,
    password_reset_tokens,
    platform_groups,
    salla_webhooks,
    sessions,
    users,
);
