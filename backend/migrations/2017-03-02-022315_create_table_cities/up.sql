CREATE TABLE IF NOT EXISTS sso.cities
(
  id serial NOT NULL,
  name character varying(256) NOT NULL,
  province_id integer NOT NULL,
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT cities_pkey PRIMARY KEY (id),
  CONSTRAINT cities_name_key UNIQUE (name),
  CONSTRAINT cities_province_id_fkey FOREIGN KEY (province_id) REFERENCES sso.provinces (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);