-- Course Prerequisites Table
CREATE TABLE IF NOT EXISTS prerequisites (
    id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS required_prerequisites (
    prerequisite_id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    CONSTRAINT pk_required_prerequisites PRIMARY KEY (prerequisite_id, course_id),
    CONSTRAINT fk_required_prerequisites_to_prerequisites FOREIGN KEY (prerequisite_id)
        REFERENCES prerequisites (id)
        ON DELETE CASCADE,
    CONSTRAINT fk_required_prerequisites_to_courses FOREIGN KEY (course_id)
        REFERENCES courses (id)
        ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS optional_prerequisites (
    prerequisite_id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    CONSTRAINT pk_optional_prerequisites PRIMARY KEY (prerequisite_id, course_id),
    CONSTRAINT fk_optional_prerequisites_to_prerequisites FOREIGN KEY (prerequisite_id)
        REFERENCES prerequisites (id)
        ON DELETE CASCADE,
    CONSTRAINT fk_optional_prerequisites_to_courses FOREIGN KEY (course_id)
        REFERENCES courses (id)
        ON DELETE SET NULL
);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_courses AS
SELECT
    c.id AS id,
    c.catalog_number AS catalog_number,
    c.subject_code AS subject_code,
    c.title AS title,
    c.external_id AS external_id,
    c.academic_level AS academic_level,
    c.description AS description,
    c.requirements AS requirements_description,
    (
        SELECT
            COALESCE(NULLIF(jsonb_agg(DISTINCT co.*)::TEXT, '[null]'), '[]')
                AS
                offerings
        FROM
            course_offerings co
        WHERE
            c.id = co.course_id AND
            co.year = (SELECT EXTRACT(YEAR FROM CURRENT_DATE) AS current_year)
        LIMIT 1
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
GROUP BY
    c.id,
    c.subject_code
ORDER BY
    c.catalog_number;

-- Courses Courses MV Indices
CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_courses_subject_code_catalog_number
    ON mv_courses (subject_code, catalog_number);

CREATE INDEX IF NOT EXISTS idx_mv_courses_title
    ON mv_courses (title);

CREATE INDEX IF NOT EXISTS idx_mv_courses_academic_level
    ON mv_courses (academic_level);

CREATE INDEX IF NOT EXISTS idx_mv_courses_id_subject_code
    ON mv_courses (id, subject_code);

REFRESH MATERIALIZED VIEW mv_courses;
