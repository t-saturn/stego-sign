CREATE SCHEMA IF NOT EXISTS files;

CREATE TABLE IF NOT EXISTS files.buckets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name TEXT NOT NULL UNIQUE,
    region TEXT NOT NULL DEFAULT 'us-east-1',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
);

CREATE TABLE IF NOT EXISTS files.objects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    bucket_id UUID NOT NULL REFERENCES files.buckets (id) ON DELETE RESTRICT,
    object_key TEXT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now (),
    UNIQUE (bucket_id, object_key)
);

CREATE INDEX IF NOT EXISTS idx_objects_bucket_id ON files.objects (bucket_id);

CREATE INDEX IF NOT EXISTS idx_objects_uploaded_at ON files.objects (uploaded_at DESC);