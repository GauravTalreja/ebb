SELECT
    t.name as name,
    (
        SELECT
            COUNT(*)
        FROM
            course_tags ct
        WHERE
                t.id = ct.tag_id
    ) AS "course_count!: _"
FROM
    tags t
ORDER BY
    "course_count!: _" DESC;
