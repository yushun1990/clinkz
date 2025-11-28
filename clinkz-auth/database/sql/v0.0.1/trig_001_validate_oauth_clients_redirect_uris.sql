CREATE OR REPLACE FUNCTION validate_redirect_uris ()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM jsonb_array_elements(NEW.redirect_uris::jsonb);
    RETURN NEW;
EXCEPTION WHEN others THEN
    RAISE EXCEPTION 'redirect_uris must be a valid JSON array of strings';
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER trig_validate_oauth_client_uris
    BEFORE INSERT OR UPDATE ON oauth_clients
    FOR EACH ROW EXECUTE FUNCTION validate_redirect_uris();
