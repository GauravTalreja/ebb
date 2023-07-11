SELECT
    c.id AS "course_id!",
    c.catalog_number as course_catalog_number,
    c.subject_code AS course_subject_code,
    year AS "year!",
    term AS "term!",
    COALESCE(
        NULLIF(jsonb_agg(DISTINCT cs.*)::TEXT, '[null]'),
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
