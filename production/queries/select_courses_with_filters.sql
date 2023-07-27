SELECT
    c.id,
    c.catalog_number,
    c.subject_code,
    c.title,
    c.external_id,
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
    LEFT JOIN class_schedule cs ON co.id = cs.course_offering_id
WHERE
    c.subject_code || c.catalog_number ILIKE $2
    AND c.catalog_number ILIKE ANY( $3 )
    AND (
        (NOT $4)
        OR
        ($4 AND ($5 AND cs.monday = $5) OR ($6 AND cs.tuesday = $6) OR ($7 AND cs.wednesday = $7)
            OR ($8 AND cs.thursday = $8) OR ($9 AND cs.friday = $9) OR ($10 AND cs.saturday = $10))
    )
    AND (
        (NOT $11)
        OR
        ($11 AND ($12 AND cs.start_time >= TIME '0:00:00' AND cs.start_time < TIME '12:00:00')
            OR ($13 AND cs.start_time >= TIME '12:00:00' AND cs.start_time < TIME '17:00:00')
            OR ($14 AND cs.start_time >= TIME '17:00:00' AND cs.start_time < TIME '24:00:00'))
    )
    AND (
        (NOT $15 AND cs.current_enrollment < cs.max_enrollment)
        OR
        ($15)
    )
    AND co.term ILIKE $16
GROUP BY
    c.id,
    c.subject_code
ORDER BY
    c.catalog_number;
