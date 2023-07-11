SELECT
    c.id AS "course_id!: _",
    c.subject_code AS course_subject_code,
    c.catalog_number as course_catalog_number,
    year AS "year!: _",
    term AS "term!: _",
    COALESCE(
        NULLIF(json_agg(DISTINCT cs.*)::TEXT, '[null]'),
        '[]'
    )::jsonb AS "schedules!: _"
FROM
    course_offerings co
    LEFT JOIN courses c ON c.id = co.course_id
    LEFT JOIN class_schedule cs ON cs.course_offering_id = co.id
WHERE
    UPPER(c.subject_code) || c.catalog_number::VARCHAR = UPPER($1)
GROUP BY
    c.id,
    co.id;
