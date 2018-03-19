-- Table: sso.genders

CREATE TABLE IF NOT EXISTS sso.genders
(
  id serial NOT NULL,
  name character varying(128) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT genders_pkey PRIMARY KEY (id),
  CONSTRAINT genders_name_key UNIQUE (name)
)
WITH (
  OIDS=FALSE
);