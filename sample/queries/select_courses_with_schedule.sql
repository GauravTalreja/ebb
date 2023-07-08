SELECT
    c.id,
    c.catalog_number,
    c.subject_code,
    c.title,
    c.external_id,
(
        SELECT
            COALESCE(NULLIF(jsonb_agg(DISTINCT co.*), '[null]'), '[]'::jsonb),
            (
                SELECT
                    COALESCE(NULLIF(jsonb_agg(DISTINCT cs.*), '[null]'), '[]'::jsonb),
                FROM
                    course_schedule cs
                WHERE
                    cs.course_offering_id = co.id
            ) AS "course_schedule!: _"
        FROM
            course_offerings co
        WHERE
            c.id = co.course_id
            AND co.year = $1) AS "offerings!: _"
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
