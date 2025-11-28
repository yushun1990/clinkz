CREATE TABLE IF NOT EXISTS jwk_keys (
    key_id          TEXT            PRIMARY KEY,
    private_key     TEXT            NOT NULL,
    public_key      TEXT            NOT NULL,
    algorithm       TEXT            NOT NULL DEFAULT 'RS256',
    active          BOOLEAN         NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT now()
);
