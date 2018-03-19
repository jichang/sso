-- Table: sso.scopes

CREATE TABLE IF NOT EXISTS sso.scopes
(
  id bigserial NOT NULL,
  application_id bigint NOT NULL,
  name character varying(128) NOT NULL,
  description character varying(1024) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT scopes_pkey PRIMARY KEY (id),
  CONSTRAINT scopes_name_key UNIQUE (name),
  CONSTRAINT scopes_application_id_fkey FOREIGN KEY (application_id) REFERENCES sso.applications (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);