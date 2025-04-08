-- Sequences
CREATE SEQUENCE IF NOT EXISTS integrated_stores_id_seq;

-- Table Definition
CREATE TABLE integrated_stores (
   "id" int8 NOT NULL DEFAULT nextval('integrated_stores_id_seq'::regclass),
    "user_id" int8 NOT NULL,
    "integration_platform_id" int8 NOT NULL,
    "is_stopped" bool NOT NULL DEFAULT false,
    "is_disabled" bool NOT NULL DEFAULT false,
    "store_url" varchar NOT NULL,
    "store_name" varchar NOT NULL,
    "token" text,
    "refresh_token" text,
    "shop_id" varchar,
    "default_carrier_id" int8,
    "default_shipping_address_id" int8,
    "integration_info" text,
    "created_at" timestamptz,
    "updated_at" timestamptz,
    "integration_token_data" text,
    "odd_enabled" bool NOT NULL DEFAULT false,
    "whpm_enabled" bool NOT NULL DEFAULT false,
    "webhook_authorization" text NOT NULL DEFAULT md5((random())::text),
    "account_id" int8,
    "reqs_mapper_class" varchar,
    "integration_abilities" json NOT NULL DEFAULT '["create_awbs_api"]'::json,
    "order_proc_method" varchar,
    "group_id" int8,
    "auth_code" varchar,
    -- "default_carrier_service_id" int8,
    "authorization_code" text,
    PRIMARY KEY ("id")
);