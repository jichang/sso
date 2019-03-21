CREATE TABLE IF NOT EXISTS sso.preferences
(
  id serial NOT NULL,
  key integer NOT NULL,
  created_time timestamp with time zone NOT NULL default now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer DEFAULT 0,
  CONSTRAINT preferences_pkey PRIMARY KEY (id),
  CONSTRAINT preferences_unique_key UNIQUE (key)
)
WITH (
  OIDS=FALSE
);