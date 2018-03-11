-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sso.audits
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  action_id integer NOT NULL,
  action_details json NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT audits_pkey PRIMARY KEY (id),
  CONSTRAINT audits_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT audits_action_id_fkey FOREIGN KEY (action_id) REFERENCES sso.actions (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);