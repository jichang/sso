-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sso.actions
(
  id serial NOT NULL,
  key character varying(128) NOT NULL,
  name character varying(128) NOT NULL,
  description character varying(1024) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT actions_pkey PRIMARY KEY (id),
  CONSTRAINT actions_key_key UNIQUE (key),
  CONSTRAINT actions_name_key UNIQUE (name)
)
WITH (
  OIDS=FALSE
);