SELECT
    email, username, password
FROM
    users 
WHERE 
    id = $1