CREATE TABLE IF NOT EXISTS audit_logs (
    id          UUID       PRIMARY KEY  DEFAULT uuidv7(),
    tenant_id   UUID       NOT NULL,
    user_id     UUID,
    clinet_id   UUID,
    action      TEXT       NOT NULL,
    ip_address  INET,
    user_agent  TEXT,
    metadata    JSONB,
    created_at  TIMESTAMPTZ NOT NULL    DEFAULT now()
);
