CREATE TABLE IF NOT EXISTS sso.invitations (
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  code varchar NOT NULL,
  created_time timestamp with time zone NOT NULL default now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer DEFAULT 0,
  CONSTRAINT invitation_pkey PRIMARY KEY (id),
  CONSTRAINT invitation_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE CASCADE
)
WITH (
  OIDS=FALSE
);