UPDATE
    users
SET
    email = $1, username = $2, password = $3, updated_at = now()
WHERE
    id = $4 and deleted_at IS NULL
RETURNING email, username, password