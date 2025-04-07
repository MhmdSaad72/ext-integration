// @generated automatically by Diesel CLI.

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
