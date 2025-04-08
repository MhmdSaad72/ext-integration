-- Sequences
CREATE SEQUENCE IF NOT EXISTS carriers_id_seq;

-- Table Definition
CREATE TABLE "public"."carriers" (
    "id" int8 NOT NULL DEFAULT nextval('carriers_id_seq'::regclass),
    "carrier" varchar NOT NULL,
    "cc_price" float8 NOT NULL,
    "cod_price" float8 NOT NULL,
    "logo" varchar,
    "user_label_en" varchar NOT NULL,
    "user_label_ar" varchar NOT NULL,
    "active" bool NOT NULL DEFAULT true,
    "created_at" timestamptz,
    "updated_at" timestamptz,
    "notes" text,
    "handler_class" varchar,
    "def_service_id" int8,
    "request_urls" json,
    PRIMARY KEY ("id")
);