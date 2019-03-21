CREATE TABLE IF NOT EXISTS sso.totp
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  secret bytea NOT NULL,
  created_time timestamp with time zone NOT NULL default now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer DEFAULT 0,
  CONSTRAINT totp_pkey PRIMARY KEY (id),
  CONSTRAINT totp_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT totp_unique_key UNIQUE (user_id)
)
WITH (
  OIDS=FALSE
);