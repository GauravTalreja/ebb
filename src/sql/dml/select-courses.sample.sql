SELECT
    c.id AS id,
    c.catalog_number AS catalog_number,
    c.subject_code AS subject_code,
    c.title AS title,
    c.external_id AS external_id,
    (
        SELECT
            COALESCE(NULLIF(json_agg(DISTINCT co.*)::TEXT, '[null]'), '[]')
                AS
                offerings
        FROM
            course_offerings co
        WHERE
            c.id = co.course_id AND
            co.year = $1
        LIMIT 1
    ) AS offerings
FROM
    courses c
    LEFT JOIN course_offerings co ON c.id = co.course_id
WHERE
    c.subject_code || c.catalog_number::VARCHAR ILIKE $2
GROUP BY
    c.id,
    c.subject_code
ORDER BY
    c.catalog_number;
