INSERT INTO sso.group_roles (group_id, role_id)
VALUES
  (
    ( SELECT id FROM sso.groups WHERE name = 'admin'),
    ( SELECT id FROM sso.roles WHERE name = 'admin')
  ),
  (
    ( SELECT id FROM sso.groups WHERE name = 'normal'),
    ( SELECT id FROM sso.roles WHERE name = 'normal')
  ),
  (
    ( SELECT id FROM sso.groups WHERE name = 'guest'),
    ( SELECT id FROM sso.roles WHERE name = 'guest')
  );