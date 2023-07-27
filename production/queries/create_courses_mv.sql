CREATE MATERIALIZED VIEW IF NOT EXISTS mv_courses AS
SELECT
    c.id AS id,
    c.catalog_number AS catalog_number,
    c.subject_code AS subject_code,
    c.title AS title,
    c.external_id AS external_id,
    c.academic_level AS academic_level,
    c.description AS description,
    c.requirements AS requirements_description,
    (
        SELECT
            COALESCE(NULLIF(jsonb_agg(DISTINCT co.*)::TEXT, '[null]'), '[]')
                AS
                offerings
        FROM
            course_offerings co
        WHERE
            c.id = co.course_id AND
            co.year = (SELECT EXTRACT(YEAR FROM CURRENT_DATE) AS current_year)
        LIMIT 1
    ) AS offerings,
    COALESCE(
        NULLIF(array_agg(DISTINCT rc.subject_code || rc.catalog_number::TEXT), '{null}'),
        '{}'
    ) AS required_prerequisites,
    COALESCE(
        NULLIF(array_agg(DISTINCT oc.subject_code || oc.catalog_number::TEXT), '{null}'),
        '{}'
    ) AS optional_prerequisites
FROM
    courses c
    LEFT JOIN course_offerings co ON c.id = co.course_id
    LEFT JOIN prerequisites p ON c.prerequisites_id = p.id
    LEFT JOIN required_prerequisites rp ON p.id = rp.prerequisite_id
    LEFT JOIN courses rc ON rp.course_id = rc.id
    LEFT JOIN optional_prerequisites op ON p.id = op.prerequisite_id
    LEFT JOIN courses oc ON op.course_id = oc.id
GROUP BY
    c.id,
    c.subject_code
ORDER BY
    c.catalog_number;

REFRESH MATERIALIZED VIEW mv_courses;
