CREATE TABLE IF NOT EXISTS sso.permissions
(
  id serial NOT NULL,
  resource_type integer NOT NULL,
  action_type integer NOT NULL,
  created_time timestamp with time zone NOT NULL default now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer DEFAULT 0,
  CONSTRAINT permissions_pkey PRIMARY KEY (id),
  CONSTRAINT permissions_unique_key UNIQUE (resource_type, action_type)
)
WITH (
  OIDS=FALSE
);