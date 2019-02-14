INSERT INTO sso.role_permissions(role_id, permission_id)
VALUES
  (
    (SELECT id FROM sso.roles WHERE name = 'admin'),
    (SELECT id FROM sso.permissions WHERE resource_type = 1 AND action_type = 1)
  ),
  (
    (SELECT id FROM sso.roles WHERE name = 'admin'),
    (SELECT id FROM sso.permissions WHERE resource_type = 1 AND action_type = 2)
  ),
  (
    (SELECT id FROM sso.roles WHERE name = 'admin'),
    (SELECT id FROM sso.permissions WHERE resource_type = 1 AND action_type = 3)
  ),
  (
    (SELECT id FROM sso.roles WHERE name = 'admin'),
    (SELECT id FROM sso.permissions WHERE resource_type = 1 AND action_type = 4)
  );