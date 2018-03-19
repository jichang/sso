-- Table: sso.roles

CREATE TABLE IF NOT EXISTS sso.group_users
(
  group_id bigint NOT NULL,
  user_id bigint NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT group_users_pkey PRIMARY KEY (user_id, group_id),
  CONSTRAINT group_users_group_id_fkey FOREIGN KEY (group_id) REFERENCES sso.groups (id) ON UPDATE NO ACTION ON DELETE NO ACTION, 
  CONSTRAINT group_users_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);