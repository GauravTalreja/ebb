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
