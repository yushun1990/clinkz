CREATE TABLE IF NOT EXISTS device_authorizations (
    device_code            TEXT                  PRIMARY KEY,
    user_code              TEXT                  UNIQUE NOT NULL,
    tenant_id              UUID                  NOT NULL,
    session_id             UUID,
    client_id              UUID                  NOT NULL,
    user_id                UUID,
    scopes                 TEXT[]                NOT NULL,
    verified               BOOLEAN               DEFAULT false,
    expires_at             TIMESTAMPTZ           NOT NULL,
    created_at             TIMESTAMPTZ           NOT NULL DEFAULT now(),
    INDEX idx_device_user_code(user_code)
);
