SELECT
    DISTINCT c.id AS "id!: _",
    c.catalog_number AS "catalog_number!: _",
    c.subject_code AS "subject_code!: _",
    c.title AS "title!: _",
    c.external_id AS "external_id!: _",
    c.offerings::jsonb AS "offerings!: _"
FROM
    mv_courses c
    INNER JOIN course_tags ct ON c.id = ct.course_id
    INNER JOIN tags t ON ct.tag_id = t.id
WHERE
    t.name SIMILAR TO $1
GROUP BY
    c.id,
    c.title,
    c.external_id,
    c.subject_code,
    c.catalog_number,
    c.offerings;
