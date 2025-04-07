-- Sequences
CREATE SEQUENCE IF NOT EXISTS salla_webhooks_id_seq;

-- Table Definition
CREATE TABLE salla_webhooks (
    "id" int8 NOT NULL DEFAULT nextval('salla_webhooks_id_seq'::regclass),
    "salla_plugin_name" varchar NOT NULL DEFAULT 'Storage Station fulfillment'::character varying,
    "salla_plugin_id" int8 NOT NULL DEFAULT '1737468315'::bigint,
    "event" varchar NOT NULL,
    "merchant_id" int8 NOT NULL,
    "order_id" int8,
    "order_reference_id" int8,
    "payload" jsonb NOT NULL,
    "processed" bool NOT NULL DEFAULT false,
    "created_at" timestamp,
    "updated_at" timestamp,
    "deleted_at" timestamp,
    PRIMARY KEY ("id")
);