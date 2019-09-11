ALTER TABLE sso.applications
ADD COLUMN client_id bytea,
ADD COLUMN client_secret bytea;

WITH secrets AS (
  SELECT application_id, client_id, client_secret
  FROM sso.application_secrets
)
UPDATE sso.applications
SET client_id = secrets.client_id,
    client_secret = secrets.client_secret
FROM secrets
WHERE secrets.application_id = sso.applications.id;

ALTER TABLE sso.applications
ALTER COLUMN client_id SET NOT NULL,
ALTER COLUMN client_secret SET NOT NULL;

DROP TABLE sso.application_secrets;