-- This file should undo anything in `up.sql`
DELETE FROM sso.actions
 WHERE key = 'users.signup';
DELETE FROM sso.actions
 WHERE key = 'users.signin';
DELETE FROM sso.actions
 WHERE key = 'users.contacts.create';
DELETE FROM sso.actions
 WHERE key = 'users.contacts.update';
DELETE FROM sso.actions
 WHERE key = 'users.contacts.remove';
DELETE FROM sso.actions
 WHERE key = 'users.applications.create';
DELETE FROM sso.actions
 WHERE key = 'users.applications.update';
DELETE FROM sso.actions
 WHERE key = 'users.applications.remove';
DELETE FROM sso.actions
 WHERE key = 'users.authorizations.create';
DELETE FROM sso.actions
 WHERE key = 'users.authorizations.update';
DELETE FROM sso.actions
 WHERE key = 'users.authorizations.remove';