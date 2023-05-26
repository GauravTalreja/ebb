CREATE TABLE IF NOT EXISTS courses (
    id serial primary key,
    name varchar(40) not null unique,
    department varchar(30) not null
);

insert INTO courses (name, department) values
    ('Math', 'Mathematics'),
    ('Physics', 'Physics'),
    ('Chemistry', 'Chemistry'),
    ('English', 'Language Arts'),
    ('History', 'Social Sciences'),
    ('Biology', 'Life Sciences'),
    ('Computer Science', 'Computer Science'),
    ('Art', 'Fine Arts'),
    ('Economics', 'Social Sciences'),
    ('Psychology', 'Social Sciences');
