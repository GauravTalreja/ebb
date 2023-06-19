SELECT
    id,
    name,
    department
FROM
    courses
WHERE
    name ILIKE $ 1
