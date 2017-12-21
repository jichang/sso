CREATE TABLE IF NOT EXISTS sso.contact_types
(
  id serial NOT NULL,
  name character varying(128) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT contact_types_pkey PRIMARY KEY (id),
  CONSTRAINT contact_types_name_key UNIQUE (name)
)
WITH (
  OIDS=FALSE
);