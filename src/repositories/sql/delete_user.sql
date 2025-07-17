UPDATE 
    users 
SET 
    deleted_at = now() 
WHERE
    id = $1 and deleted_at IS NULL
RETURNING deleted_at