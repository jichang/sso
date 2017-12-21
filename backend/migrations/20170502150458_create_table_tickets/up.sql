-- Table: sso.tickets

CREATE TABLE IF NOT EXISTS sso.tickets
(
  id bigserial NOT NULL,
  authorization_id bigint NOT NULL,
  open_id character varying(256) NOT NULL,
  access_token bytea NOT NULL,
  refresh_token bytea NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT tickets_pkey PRIMARY KEY (id),
  CONSTRAINT tickets_authorization_id_fkey FOREIGN KEY (authorization_id) REFERENCES sso.authorizations (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);