CREATE TABLE IF NOT EXISTS authorization_codes (
    code                   TEXT          PRIMARY KEY,
    tenant_id              UUID          NOT NULL,
    session_id             UUID          NOT NULL,
    client_id              UUID          NOT NULL,
    user_id                UUID          NOT NULL,
    redirect_uri           TEXT          NOT NULL,
    scopes                 TEXT[]        NOT NULL,
    code_challenge         TEXT          NOT NULL,
    code_challenge_method  TEXT          DEFAULT 'S256',
    expires_at             TIMESTAMPTZ   NOT NULL,
    created_at             TIMESTAMPTZ   NOT NULL DEFAULT now(),
    INDEX idx_auth_codes_expires (expires_at)
);
