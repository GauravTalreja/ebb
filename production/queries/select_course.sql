SELECT
    id AS "id!",
    catalog_number AS "catalog_number!",
    subject_code AS "subject_code!",
    external_id AS "external_id!",
    academic_level AS "academic_level!",
    title AS "title!",
    description AS "description!",
    requirements_description AS "requirements_description",
    required_prerequisites AS "required_prerequisites!: _",
    optional_prerequisites AS "optional_prerequisites!: _"
FROM
    mv_courses c
WHERE
    UPPER(c.subject_code) || c.catalog_number::VARCHAR = UPPER($1)
GROUP BY
    c.id,
    c.catalog_number,
    c.subject_code,
    c.external_id,
    c.academic_level,
    c.title,
    c.description,
    c.requirements_description,
    c.required_prerequisites,
    c.optional_prerequisites
LIMIT 1;
