CREATE TABLE IF NOT EXISTS sso.audits
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  client_addr varchar NOT NULL,
  happened_time timestamp with time zone NOT NULL DEFAULT now(),
  type integer NOT NULL,
  details json,
  CONSTRAINT audits_pkey PRIMARY KEY (id)
)
WITH (
  OIDS=FALSE
);