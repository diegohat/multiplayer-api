SELECT
    email, username, password
FROM
    users 
WHERE 
    id = $1 and deleted_at IS NULL