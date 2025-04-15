-- Sequences
CREATE SEQUENCE IF NOT EXISTS temp_stores_integrations_id_seq;

-- Table Definition
CREATE TABLE temp_stores_integrations (
    "id" int8 NOT NULL DEFAULT nextval('temp_stores_integrations_id_seq'::regclass),
    "store_name" varchar NOT NULL,
    "store_url" varchar NOT NULL,
    "email" varchar NOT NULL,
    "shop_id" int4 NOT NULL,
    "access_token" text NOT NULL,
    "refresh_token" text NOT NULL,
    "expires" int8 NOT NULL,
    "default_carrier_id" int4 NOT NULL,
    "integration_platform_id" int4 NOT NULL,
    "odd_enabled" bool NOT NULL,
    "new_store_id" int8,
    "new_assigned_user_id" int8,
    "created_at" timestamptz,
    "updated_at" timestamptz,
    "auth_code" varchar,
    "authorization_code" text,
    PRIMARY KEY ("id")
);