// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "availability_status"))]
    pub struct AvailabilityStatus;
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

diesel::allow_tables_to_appear_in_same_query!(
    carriers,
    integrated_stores,
    integration_platforms,
    platform_groups,
    salla_webhooks,
);
