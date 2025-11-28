CREATE TABLE IF NOT EXISTS oauth_clients (
    id                 UUID         PRIMARY KEY     DEFAULT uuidv7(),
    tenant_id          UUID         NOT NULL,
    client_id          UUID         NOT NULL,
    client_secret      TEXT,
    client_name        TEXT         NOT NULL,
    redirect_uris      JSONB        NOT NULL        DEFAULT '[]'::jsonb,
    grant_types        TEXT[]       NOT NULL        DEFAULT '{oauthorization_code,refresh_token,client_credentials}',
    scopes             TEXT[]       NOT NULL        DEFAULT '{openid,profile,email,offline_access}',
    is_confidential    BOOLEAN      NOT NULL        DEFAULT true,
    created_by         UUID         NOT NULL,
    created_at         TIMESTAMPTZ  NOT NULL        DEFAULT now(),
    updated_by         UUID         NOT NULL,
    updated_at         TIMESTAMPTZ  NOT NULL        DEFAULT now(),
    deleted_by         UUID         NULL,
    deleted_at         TIMESTAMPTZ  NULL
);
