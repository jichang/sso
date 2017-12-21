-- Table: sso.applications

CREATE TABLE IF NOT EXISTS sso.applications
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  name character varying(1024) NOT NULL,
  website_uri character varying(2048),
  client_id bytea NOT NULL,
  client_secret bytea NOT NULL,
  callback_uri varchar(2048) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT applications_pkey PRIMARY KEY (id),
  CONSTRAINT applications_name_key UNIQUE (name),
  CONSTRAINT applications_client_id_key UNIQUE (client_id),
  CONSTRAINT applications_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);