CREATE MATERIALIZED VIEW IF NOT EXISTS mv_courses AS
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
                co.year = (SELECT EXTRACT(YEAR FROM CURRENT_DATE) AS current_year)
        LIMIT 1
    ) AS offerings
FROM
    courses c
    LEFT JOIN course_offerings co ON c.id = co.course_id
GROUP BY
    c.id,
    c.subject_code
ORDER BY
    c.catalog_number;

REFRESH MATERIALIZED VIEW mv_courses;
