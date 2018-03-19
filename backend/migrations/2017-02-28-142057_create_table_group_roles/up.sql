-- Table: sso.roles

CREATE TABLE IF NOT EXISTS sso.group_roles
(
  group_id bigint NOT NULL,
  role_id bigint NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT group_roles_pkey PRIMARY KEY (group_id, role_id),
  CONSTRAINT group_roles_group_id_fkey FOREIGN KEY (group_id) REFERENCES sso.groups (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT group_roles_role_id_fkey FOREIGN KEY (role_id) REFERENCES sso.roles (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);