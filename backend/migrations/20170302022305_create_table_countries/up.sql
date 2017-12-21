CREATE TABLE IF NOT EXISTS sso.countries
(
  id serial NOT NULL,
  name character varying(256) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT countries_pkey PRIMARY KEY (id),
  CONSTRAINT countries_name_key UNIQUE (name)
)
WITH (
  OIDS=FALSE
);