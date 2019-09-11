CREATE TABLE IF NOT EXISTS sso.application_secrets
(
  id bigserial NOT NULL,
  application_id bigint NOT NULL,
  client_id bytea NOT NULL,
  client_secret bytea NOT NULL,
  created_time timestamp with time zone NOT NULL default now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT application_secrets_pkey PRIMARY KEY (id),
  CONSTRAINT application_secrets_client_id_secret_key UNIQUE (client_id, client_secret),
  CONSTRAINT application_secrets_application_id_fkey FOREIGN KEY (application_id) REFERENCES sso.applications (id) ON UPDATE NO ACTION ON DELETE CASCADE
)
WITH (
  OIDS=FALSE
);

INSERT INTO sso.application_secrets(application_id, client_id, client_secret)
SELECT id, client_id, client_secret
FROM sso.applications;

ALTER TABLE sso.applications DROP COLUMN client_id, DROP COLUMN client_secret;
