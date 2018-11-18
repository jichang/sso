-- Table: sso.accounts

CREATE TABLE IF NOT EXISTS sso.accounts
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  username character varying(256) NOT NULL,
  salt character varying(32) NOT NULL,
  hash bytea NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT auth_pkey PRIMARY KEY (id),
  CONSTRAINT auth_user_id_key UNIQUE (user_id),
  CONSTRAINT auth_username_key UNIQUE (username),
  CONSTRAINT auth_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE CASCADE
)
WITH (
  OIDS=FALSE
);