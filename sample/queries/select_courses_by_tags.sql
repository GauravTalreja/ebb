SELECT
    DISTINCT c.id AS id,
    c.catalog_number AS catalog_number,
    c.subject_code AS subject_code,
    c.external_id AS external_id,
    c.title AS title,
    (
        SELECT
            COALESCE(NULLIF(jsonb_agg(DISTINCT co.*), '[null]'), '[]'::jsonb)
        FROM
            course_offerings co
        WHERE
            c.id = co.course_id
            AND co.year = $1
    ) AS "offerings!: _"
FROM
    courses c
    LEFT JOIN course_offerings co ON c.id = co.course_id
    INNER JOIN course_tags ct ON c.id = ct.course_id
    INNER JOIN tags t ON ct.tag_id = t.id
WHERE
    t.name SIMILAR TO $2
GROUP BY
    c.id;

