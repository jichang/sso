DELETE
FROM sso.group_roles
WHERE group_id in (SELECT id FROM sso.groups);