SELECT
    c.id AS id,
    c.catalog_number AS catalog_number,
    c.subject_code AS subject_code,
    c.external_id AS external_id,
    c.academic_level AS academic_level,
    c.title AS title,
    c.description AS description,
    COALESCE(
        NULLIF(json_agg(DISTINCT co.*)::TEXT, '[null]'),
        '[]'
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
WHERE
    c.subject_code || c.catalog_number::VARCHAR = $1
GROUP BY
    c.id
LIMIT 1;
