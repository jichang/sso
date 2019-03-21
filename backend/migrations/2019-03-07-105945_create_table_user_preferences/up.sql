CREATE TABLE IF NOT EXISTS sso.user_preferences
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  preference_key integer NOT NULL,
  enabled boolean NOT NULL,
  details json,
  created_time timestamp with time zone NOT NULL default now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer DEFAULT 0,
  CONSTRAINT user_preferences_pkey PRIMARY KEY (id),
  CONSTRAINT user_preferences_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT user_preferences_preference_key_fkey FOREIGN KEY (preference_key) REFERENCES sso.preferences (key) ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT user_preferences_unique_key UNIQUE (user_id, preference_key)
)
WITH (
  OIDS=FALSE
);