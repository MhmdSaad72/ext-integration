-- Sequences
CREATE SEQUENCE IF NOT EXISTS platform_groups_id_seq;

-- Create enum type (availability_status)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'availability_status') THEN
        CREATE TYPE ip_availability_status AS ENUM (
            'available', 
            'not_available', 
            'soon'
        );
    END IF;
END$$;

-- Table Definition
CREATE TABLE platform_groups (
    "id" int8 NOT NULL DEFAULT nextval('platform_groups_id_seq'::regclass),
    "name" varchar,
    "label" varchar,
    "img" varchar,
    "img_light" varchar,
    "img_max_height" varchar,
    "availability_status" availability_status NOT NULL DEFAULT 'not_available'::availability_status,
    "showable" bool NOT NULL DEFAULT false,
    "created_at" timestamp,
    "updated_at" timestamp,
    PRIMARY KEY ("id")
);
