-- Table: sso.roles

CREATE TABLE IF NOT EXISTS sso.groups
(
  id bigserial NOT NULL,
  name character varying(128) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT groups_pkey PRIMARY KEY (id),
  CONSTRAINT groups_name_key UNIQUE (name)
)
WITH (
  OIDS=FALSE
);