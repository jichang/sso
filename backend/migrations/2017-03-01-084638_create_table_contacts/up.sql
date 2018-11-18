-- Table: sso.contacts

CREATE TABLE IF NOT EXISTS sso.contacts
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  type_id integer NOT NULL,
  identity character varying(2048) NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  verified_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT contacts_pkey PRIMARY KEY (id),
  CONSTRAINT contacts_identity_key UNIQUE (identity),
  CONSTRAINT contacts_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT contacts_type_id_fkey FOREIGN KEY (type_id) REFERENCES sso.contact_types (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);