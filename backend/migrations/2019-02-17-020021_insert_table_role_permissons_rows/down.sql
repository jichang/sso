DELETE FROM sso.role_permissions
USING sso.permissions as permissions
LEFT JOIN sso.permissions as permissions ON role_permissions.permission_id = permissions.id
WHERE permissions.resource_type IN (2, 3, 4, 5, 6, 7);