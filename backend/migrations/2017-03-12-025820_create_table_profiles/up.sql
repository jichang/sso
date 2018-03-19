-- Table: sso.profiles

CREATE TABLE IF NOT EXISTS sso.profiles
(
  id bigserial NOT NULL,
  user_id bigint NOT NULL,
  gender_id integer,
  name character varying(2048),
  birthday timestamp with time zone,
  country_id integer,
  province_id integer,
  city_id integer,
  introduction character varying(2048),
  created_time timestamp with time zone NOT NULL DEFAULT now(),
  updated_time timestamp with time zone,
  removed_time timestamp with time zone,
  status integer NOT NULL DEFAULT 0,
  CONSTRAINT profiles_pkey PRIMARY KEY (id),
  CONSTRAINT profiles_user_id_key UNIQUE (user_id),
  CONSTRAINT profiles_user_id_fkey FOREIGN KEY (user_id) REFERENCES sso.users (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT profiles_gender_id_fkey FOREIGN KEY (gender_id) REFERENCES sso.genders (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT profiles_country_id_fkey FOREIGN KEY (country_id) REFERENCES sso.countries (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT profiles_province_id_fkey FOREIGN KEY (province_id) REFERENCES sso.provinces (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT profiles_city_id_fkey FOREIGN KEY (city_id) REFERENCES sso.cities (id) ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);