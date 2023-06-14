CREATE TABLE IF NOT EXISTS public.courses (
    id SERIAL PRIMARY KEY,
    catalog_number SMALLINT NOT NULL,
    subject_code VARCHAR(10) NOT NULL,
    external_id VARCHAR(10) NOT NULL,
    academic_level VARCHAR(13) NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    prerequisites_id INTEGER NULL,
    CONSTRAINT uk_courses_external_id UNIQUE (external_id),
    CONSTRAINT uk_courses_catalog_subject UNIQUE (catalog_number, subject_code),
    CONSTRAINT fk_courses_to_prerequisites FOREIGN KEY (prerequisites_id)
        REFERENCES prerequisites (id)
        ON DELETE SET NULL,
    CONSTRAINT chk_courses_external_id_not_empty CHECK (external_id <> ''),
    CONSTRAINT chk_courses_subject_code_not_empty CHECK (subject_code <> ''),
    CONSTRAINT chk_courses_academic_level CHECK (academic_level IN ('undergraduate', 'graduate')),
    CONSTRAINT chk_courses_title_not_empty CHECK (title <> '')
);

-- Course Offerings Table
CREATE TABLE IF NOT EXISTS course_offerings (
    id SERIAL PRIMARY KEY,
    course_id INTEGER NOT NULL,
    year SMALLINT NOT NULL,
    term VARCHAR(6) NOT NULL,
    max_enrollment SMALLINT NOT NULL DEFAULT 0,
    current_enrollment SMALLINT NOT NULL DEFAULT 0,
    CONSTRAINT uk_course_offerings_unique_data UNIQUE (course_id, year, term),
    CONSTRAINT fk_course_offerings_to_courses FOREIGN KEY (course_id)
        REFERENCES courses (id)
        ON DELETE CASCADE,
    CONSTRAINT chk_course_offerings_term CHECK (term IN ('winter', 'fall', 'spring')),
    CONSTRAINT chk_course_current_enrollment CHECK (current_enrollment <= max_enrollment)
);

CREATE TABLE IF NOT EXISTS course_schedule (
    id SERIAL PRIMARY KEY,
    section VARCHAR(30) NOT NULL,
    start_time TIME,
    end_time TIME,
    meeting_days VARCHAR(7) NOT NULL,
    instructor_name VARCHAR(50) NOT NULL,
    room_name VARCHAR(30) NOT NULL,
    campus_name VARCHAR(50) NOT NULL,
    course_offering_id INTEGER NOT NULL,
    max_enrollment SMALLINT NOT NULL DEFAULT 0,
    current_enrollment SMALLINT NOT NULL DEFAULT 0,
    CONSTRAINT fk_course_schedule_to_courses FOREIGN KEY (course_offering_id)
        REFERENCES course_offerings (id)
        ON DELETE CASCADE,
    CONSTRAINT chk_meeting_days CHECK (meeting_days ~ '^[YN]{7}$'),
    CONSTRAINT chk_course_schedule_current_enrollment CHECK (current_enrollment <= max_enrollment),
    CONSTRAINT chk_course_schedule_times CHECK (end_time > start_time)
);
