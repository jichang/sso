CREATE TABLE IF NOT EXISTS sso.users
(
  id bigserial NOT NULL,
  role_id integer NOT NULL,
  union_id uuid NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT users_pkey PRIMARY KEY (id),
  CONSTRAINT users_union_id_key UNIQUE (union_id),
  CONSTRAINT users_role_id_fkey FOREIGN KEY (role_id) REFERENCES sso.roles (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);