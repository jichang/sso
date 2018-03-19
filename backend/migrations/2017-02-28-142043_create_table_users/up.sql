CREATE TABLE IF NOT EXISTS sso.users
(
  id bigserial NOT NULL,
  union_id uuid NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT users_pkey PRIMARY KEY (id),
  CONSTRAINT users_union_id_key UNIQUE (union_id)
)
WITH (
  OIDS=FALSE
);