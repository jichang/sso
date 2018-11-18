CREATE TABLE IF NOT EXISTS sso.provinces
(
  id serial NOT NULL,
  name character varying(256) NOT NULL,
  country_id integer NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT provinces_pkey PRIMARY KEY (id),
  CONSTRAINT provinces_name_key UNIQUE (name),
  CONSTRAINT provinces_country_id_fkey FOREIGN KEY (country_id) REFERENCES sso.countries (id) ON UPDATE NO ACTION ON DELETE CASCADE
)
WITH (
  OIDS=FALSE
);