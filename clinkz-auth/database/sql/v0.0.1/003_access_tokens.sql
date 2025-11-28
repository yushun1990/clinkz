CREATE TABLE IF NOT EXISTS access_tokens (
    token           BYTEA         PRIMARY KEY,
    tenant_id       UUID          NOT NULL,
    session_id      UUID          NOT NULL,
    client_id       UUID          NOT NULL,
    user_id         UUID          NOT NULL,
    scopes          TEXT[]        NOT NULL,
    expires_at      TIMESTAMPTZ   NOT NULL,
    revoked_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ   NOT NULL          DEFAULT now()
    INDEX idx_access_tenant_session(tenant_id, session_id),
    INDEX idx_access_user(user_id),
    INDEX idx_access_expires(expires_at)
);
