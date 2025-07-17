UPDATE 
    users 
SET 
    deleted_at = now() 
WHERE
    id = $1 
RETURNING deleted_at