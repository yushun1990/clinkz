CREATE TABLE IF NOT EXISTS revoked_tokens (
    token           BYTEA                 PRIMARY KEY,
    token_type      TEXT                  NOT NULL,             -- 'access or refresh'
    revoked_at      TIMESTAMPTZ           NOT NULL DEFAULT now()
);
