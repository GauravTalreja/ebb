SELECT
    t.name as name,
    COUNT(ct.course_id) AS "course_count!: _"
FROM
    tags t
    LEFT JOIN course_tags ct ON t.id = ct.tag_id
GROUP BY
    t.name
ORDER BY
    "course_count!: _" DESC;
