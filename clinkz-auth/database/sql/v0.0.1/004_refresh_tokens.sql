CREATE TABLE IF NOT EXISTS refresh_tokens (
    id              UUID          PRIMARY KEY      DEFAULT uuidv7(),
    tenant_id       UUID          NOT NULL,
    token           BYTEA         NOT NULL,
    client_id       UUID          NOT NULL,
    user_id         UUID          NOT NULL,
    scopes          TEXT[]        NOT NULL,
    expires_at      TIMESTAMPTZ   NOT NULL,
    revoked_at      TIMESTAMPTZ   NOT NULL,
    superseded_by   UUID          REFERENCES refresh_tokens(id),
    created_at      TIMESTAMPTZ   NOT NULL          DEFAULT now()
);
