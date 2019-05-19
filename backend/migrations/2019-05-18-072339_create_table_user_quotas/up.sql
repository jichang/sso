CREATE TABLE IF NOT EXISTS sso.user_quotas
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  resource_type integer NOT NULL,
  quota integer NOT NULL,
  created_time timestamp with time zone NOT NULL default now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer DEFAULT 0,
  CONSTRAINT user_quotas_pkey PRIMARY KEY (id),
  CONSTRAINT user_quotas_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE CASCADE
)
WITH (
  OIDS=FALSE
);