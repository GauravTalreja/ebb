-- Selecting course list from single Materialized View instead of
-- more expensive query with left join across course to offerings
-- table with sub-query to aggregate offerings into an array.
--
-- Materialized view must be refreshed upon every import sync job
-- to update with latest base table data.
SELECT
    id,
    catalog_number,
    subject_code,
    title,
    external_id,
    offerings
FROM
    mv_courses
WHERE
    subject_code || catalog_number::VARCHAR ILIKE $1
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
