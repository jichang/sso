-- Your SQL goes here
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.signup', 'Signup', 'user sign up', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.accounts.create', 'create account', 'user create account', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.accounts.verify', 'verify account', 'user verify account', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.contacts.create', 'create contact', 'user create contact', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.contacts.remove', 'remove contact', 'user remove contact', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.applications.create', 'create application', 'user carete application', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.applications.update', 'update application', 'user update application', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.applications.remove', 'remove application', 'user remove application', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.authorizations.create', 'create authorization', 'user carete authorization', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.authorizations.update', 'update authorization', 'user update authorization', 1);
INSERT INTO sso.actions(key, name, description, status)
    VALUES ('users.authorizations.remove', 'remove authorization', 'user remove authorization', 1);