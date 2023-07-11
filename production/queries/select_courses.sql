-- Selecting course list from single Materialized View instead of
-- more expensive query with left join across course to offerings
-- table with sub-query to aggregate offerings into an array.
--
-- Materialized view must be refreshed upon every import sync job
-- to update with latest base table data.
SELECT
    id AS "id!",
    catalog_number AS "catalog_number!",
    subject_code AS "subject_code!",
    title AS "title!",
    external_id AS "external_id!",
    offerings::jsonb AS "offerings!: _"
FROM
    mv_courses
WHERE
    subject_code || catalog_number ILIKE $1
GROUP BY
    id,
    title,
    external_id,
    subject_code,
    catalog_number,
    offerings
ORDER BY
    catalog_number
LIMIT
    100;
